//! Minimal EventKit bridge: access, calendars, and reading/writing events.
// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).

use block2::RcBlock;
use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::Bool;
use objc2_app_kit::{NSColor, NSColorSpace};
use objc2_event_kit::{EKAlarm, EKAuthorizationStatus, EKEntityType, EKEvent, EKEventStore, EKSpan};
use objc2_foundation::{NSArray, NSDate, NSError, NSString};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub calendar_id: String,
    pub calendar: String,
    pub color: String,
    /// Unix milliseconds.
    pub start: f64,
    pub end: f64,
    pub all_day: bool,
    pub editable: bool,
    /// Reminders of the event itself, in minutes before the start (0 = at the start).
    pub alarms: Vec<i32>,
}

#[derive(Serialize)]
pub struct Calendar {
    pub id: String,
    pub title: String,
    pub color: String,
    pub writable: bool,
}

/// Payload for creating (`id` = None) or updating an event.
#[derive(Deserialize)]
pub struct EventInput {
    pub id: Option<String>,
    pub title: String,
    pub calendar_id: Option<String>,
    pub start: f64,
    pub end: f64,
    /// Minutes before the start; -1 = no reminder; absent = leave the event's reminders alone.
    #[serde(default)]
    pub reminder: Option<i32>,
}

pub fn has_access() -> bool {
    let status = unsafe { EKEventStore::authorizationStatusForEntityType(EKEntityType::Event) };
    status == EKAuthorizationStatus::FullAccess
}

/// Shows the macOS permission prompt once; the UI polls `has_access` afterwards.
pub fn request_access() {
    if has_access() {
        return;
    }
    unsafe {
        let store = EKEventStore::new();
        let done = RcBlock::new(|_granted: Bool, _err: *mut NSError| {});
        store.requestFullAccessToEventsWithCompletion(RcBlock::as_ptr(&done));
        // The store must outlive the async prompt.
        std::mem::forget(store);
        std::mem::forget(done);
    }
}

fn hex(color: &NSColor) -> String {
    let srgb = NSColorSpace::sRGBColorSpace();
    match color.colorUsingColorSpace(&srgb) {
        Some(c) => {
            let ch = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
            format!("#{:02x}{:02x}{:02x}", ch(c.redComponent()), ch(c.greenComponent()), ch(c.blueComponent()))
        }
        None => "#3b82f6".into(),
    }
}

fn date(ms: f64) -> Retained<NSDate> {
    NSDate::dateWithTimeIntervalSince1970(ms / 1000.0)
}

pub fn calendars() -> Vec<Calendar> {
    autoreleasepool(|_| unsafe {
        EKEventStore::new()
            .calendarsForEntityType(EKEntityType::Event)
            .iter()
            .map(|c| Calendar {
                id: c.calendarIdentifier().to_string(),
                title: c.title().to_string(),
                color: hex(&c.color()),
                writable: c.allowsContentModifications(),
            })
            .collect()
    })
}

fn to_event(ev: &EKEvent) -> Event {
    unsafe {
        let cal = ev.calendar();
        Event {
            id: ev.eventIdentifier().map(|s| s.to_string()).unwrap_or_default(),
            title: ev.title().to_string(),
            calendar_id: cal.as_ref().map(|c| c.calendarIdentifier().to_string()).unwrap_or_default(),
            calendar: cal.as_ref().map(|c| c.title().to_string()).unwrap_or_default(),
            color: cal.as_ref().map(|c| hex(&c.color())).unwrap_or_else(|| "#3b82f6".into()),
            start: ev.startDate().timeIntervalSince1970() * 1000.0,
            end: ev.endDate().timeIntervalSince1970() * 1000.0,
            all_day: ev.isAllDay(),
            alarms: alarm_minutes(ev),
            editable: cal.map(|c| c.allowsContentModifications()).unwrap_or(false) && organized_by_me(ev),
        }
    }
}

pub fn events(start_ms: f64, end_ms: f64) -> Vec<Event> {
    autoreleasepool(|_| unsafe {
        let store = EKEventStore::new();
        let predicate = store.predicateForEventsWithStartDate_endDate_calendars(&date(start_ms), &date(end_ms), None);
        store.eventsMatchingPredicate(&predicate).iter().map(|ev| to_event(&ev)).collect()
    })
}

/// One event by its identifier (used by the MCP server to change only the fields it was given).
pub fn event_by_id(id: &str) -> Result<Event, String> {
    autoreleasepool(|_| {
        let store = unsafe { EKEventStore::new() };
        find(&store, id).map(|ev| to_event(&ev))
    })
}

fn alarm_minutes(ev: &EKEvent) -> Vec<i32> {
    unsafe {
        let Some(alarms) = ev.alarms() else { return Vec::new() };
        let mut v: Vec<i32> = alarms
            .iter()
            .map(|a| a.relativeOffset())
            .filter(|o| *o <= 0.0)
            .map(|o| (-o / 60.0).round() as i32)
            .collect();
        v.sort();
        v.dedup();
        v
    }
}

/// Events organised by someone else (a meeting on a shared/work calendar) are read-only here:
/// moving or deleting them would only change your copy or fail on the server.
fn organized_by_me(ev: &EKEvent) -> bool {
    unsafe {
        match ev.organizer() {
            Some(o) => o.isCurrentUser(),
            None => true,
        }
    }
}

fn find(store: &EKEventStore, id: &str) -> Result<Retained<EKEvent>, String> {
    unsafe { store.eventWithIdentifier(&NSString::from_str(id)) }.ok_or_else(|| "Event not found".into())
}

/// Creates or updates an event. Only the single occurrence is changed for repeating events.
pub fn save(input: EventInput) -> Result<(), String> {
    autoreleasepool(|_| unsafe {
        let store = EKEventStore::new();
        let ev = match &input.id {
            Some(id) => {
                let ev = find(&store, id)?;
                if !organized_by_me(&ev) {
                    return Err("This meeting is organised by someone else".into());
                }
                ev
            }
            None => EKEvent::eventWithEventStore(&store),
        };
        let cal = match &input.calendar_id {
            Some(id) => store.calendarWithIdentifier(&NSString::from_str(id)),
            None => None,
        };
        if let Some(cal) = cal.or_else(|| if input.id.is_none() { store.defaultCalendarForNewEvents() } else { None }) {
            ev.setCalendar(Some(&cal));
        }
        ev.setTitle(Some(&NSString::from_str(&input.title)));
        ev.setStartDate(Some(&date(input.start)));
        ev.setEndDate(Some(&date(input.end)));
        if let Some(m) = input.reminder {
            // Replaces every reminder; "none" clears them so the calendar server adds no default one.
            if m < 0 {
                ev.setAlarms(Some(&NSArray::new()));
            } else {
                let alarm = EKAlarm::alarmWithRelativeOffset(-(m as f64) * 60.0);
                ev.setAlarms(Some(&NSArray::from_retained_slice(&[alarm])));
            }
        }
        store.saveEvent_span_error(&ev, EKSpan::ThisEvent).map_err(|e| e.localizedDescription().to_string())
    })
}

pub fn remove(id: &str) -> Result<(), String> {
    autoreleasepool(|_| unsafe {
        let store = EKEventStore::new();
        let ev = find(&store, id)?;
        if !organized_by_me(&ev) {
            return Err("This meeting is organised by someone else".into());
        }
        store.removeEvent_span_error(&ev, EKSpan::ThisEvent).map_err(|e| e.localizedDescription().to_string())
    })
}

// TEMP probe: what does the private EKEvent `color` return for synced (Google) events?
fn debug_color(ev: &EKEvent) {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;
    use std::io::Write;
    unsafe {
        let c: Option<objc2::rc::Retained<AnyObject>> = msg_send![ev, color];
        let line = match c {
            Some(o) => format!("{} | {} | {:?} | {:?}\n", ev.title(), ev.calendar().map(|c| c.title().to_string()).unwrap_or_default(), o.class(), o),
            None => format!("{} | nil\n", ev.title()),
        };
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
            .open("/private/tmp/claude-502/-Users-kacper-PhpstormProjects-sidebar-calendar/755c7397-79cd-4f87-b40e-35f0af0798d2/scratchpad/color-probe.log") {
            let _ = f.write_all(line.as_bytes());
        }
    }
}
