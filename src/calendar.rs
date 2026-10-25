//! Minimal EventKit bridge: access, calendars, and reading/writing events.
// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).

use block2::RcBlock;
use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::Bool;
use objc2_app_kit::{NSColor, NSColorSpace};
use objc2_event_kit::{EKAuthorizationStatus, EKEntityType, EKEvent, EKEventStore, EKSpan};
use objc2_foundation::{NSDate, NSError, NSString};
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
            editable: cal.map(|c| c.allowsContentModifications()).unwrap_or(false),
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
        let store = EKEventStore::new();
        find(&store, id).map(|ev| to_event(&ev))
    })
}

fn find(store: &EKEventStore, id: &str) -> Result<Retained<EKEvent>, String> {
    unsafe { store.eventWithIdentifier(&NSString::from_str(id)) }.ok_or_else(|| "Event not found".into())
}

/// Creates or updates an event. Only the single occurrence is changed for repeating events.
pub fn save(input: EventInput) -> Result<(), String> {
    autoreleasepool(|_| unsafe {
        let store = EKEventStore::new();
        let ev = match &input.id {
            Some(id) => find(&store, id)?,
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
        store.saveEvent_span_error(&ev, EKSpan::ThisEvent).map_err(|e| e.localizedDescription().to_string())
    })
}

pub fn remove(id: &str) -> Result<(), String> {
    autoreleasepool(|_| unsafe {
        let store = EKEventStore::new();
        let ev = find(&store, id)?;
        store.removeEvent_span_error(&ev, EKSpan::ThisEvent).map_err(|e| e.localizedDescription().to_string())
    })
}
