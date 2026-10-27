// UI translations. English strings are the keys; unknown strings (and unsupported languages) stay English.
// Copyright 2026 Kacper Górec. Licensed under the Apache License 2.0 (see LICENSE and NOTICE).
const I18N = {
  pl: {
    'color:red': 'Czerwony', 'color:coral': 'Koralowy', 'color:orange': 'Pomarańczowy', 'color:yellow': 'Żółty', 'color:lime': 'Limonkowy', 'color:green': 'Zielony', 'color:teal': 'Turkusowy', 'color:blue': 'Niebieski', 'color:purple': 'Fioletowy', 'color:pink': 'Różowy', 'color:brown': 'Brązowy', 'color:gray': 'Szary',
    'Settings': 'Ustawienia', 'Settings…': 'Ustawienia…', 'New event': 'Nowe wydarzenie', 'Event title': 'Tytuł wydarzenia',
    'Cancel': 'Anuluj', 'Save': 'Zapisz', 'Reload': 'Odśwież', 'Untitled': 'Bez tytułu',
    'Waiting for calendar access…': 'Oczekiwanie na dostęp do kalendarza…',
    'System Settings → Privacy & Security → Calendars': 'Ustawienia systemowe → Prywatność i ochrona → Kalendarze',
    'Show hidden events': 'Pokaż ukryte wydarzenia', 'Hide hidden events': 'Ukryj ukryte wydarzenia',
    'Use calendar colour': 'Użyj koloru kalendarza', 'Unhide event': 'Pokaż wydarzenie', 'Hide event': 'Ukryj wydarzenie',
    'Open in Calendar': 'Otwórz w Kalendarzu', 'Delete event': 'Usuń wydarzenie', 'Click again to delete': 'Kliknij ponownie, aby usunąć',
    'End must be after start': 'Koniec musi być po początku',
    'Calendar': 'Kalendarz', 'No more events': 'Brak kolejnych wydarzeń', '{t} left': 'zostało {t}', 'in {t}': 'za {t}', 'Next in {t}': 'Następne za {t}',
    'No reminder': 'Bez przypomnienia', 'At time of event': 'W momencie wydarzenia', '{n} min before': '{n} min przed',
    '{n} hour before': '{n} godz. przed', '{n} hours before': '{n} godz. przed', '{n} day before': '{n} dzień przed', '{n} days before': '{n} dni przed',
    'Off': 'Wył.', 'At start': 'W momencie',
    'Glance at your day without breaking focus.': 'Zerknij na dzień, nie odrywając się od pracy.',
    'Calendars': 'Kalendarze', 'General': 'Ogólne', 'Appearance': 'Wygląd', 'Sidebar': 'Panel boczny', 'Shortcuts': 'Skróty', 'Productivity': 'Produktywność',
    'Untick the swatch to hide a calendar. Drag the handle to reorder: higher = priority.': 'Odznacz próbkę koloru, aby ukryć kalendarz. Przeciągnij uchwyt, aby zmienić kolejność: wyżej = wyższy priorytet.',
    'Startup': 'Uruchamianie', 'Menu bar': 'Pasek menu', 'New events': 'Nowe wydarzenia', 'Reminders': 'Przypomnienia', 'Default reminder': 'Domyślne przypomnienie',
    'Preselected for new events and saved to the calendar. "Off" means no reminder there either.': 'Wstępnie wybrane dla nowych wydarzeń i zapisywane w kalendarzu. „Wył.” oznacza brak przypomnienia również tam.',
    'Theme': 'Motyw', 'Light': 'Jasny', 'Dark': 'Ciemny', 'Auto': 'Auto', 'Follow the macOS appearance': 'Zgodnie z wyglądem macOS',
    'Event style': 'Styl wydarzeń', 'Solid': 'Pełny', 'Tinted': 'Przyciemniony', 'Filled with the event colour': 'Wypełnione kolorem wydarzenia',
    'A soft tint with a colour bar on the left': 'Delikatny odcień z kolorowym paskiem po lewej',
    'Header & layout': 'Nagłówek i układ', 'Timeline': 'Oś czasu', 'Position & behaviour': 'Pozycja i zachowanie',
    'Sidebar side': 'Strona panelu', 'Left': 'Lewa', 'Right': 'Prawa', 'Sidebar width': 'Szerokość panelu', 'Reveal delay': 'Opóźnienie wysunięcia',
    'Edge preview': 'Podgląd na krawędzi', 'Leave faded entries on the screen edge while hidden': 'Zostaw wyblakłe wpisy na krawędzi ekranu, gdy panel jest ukryty',
    'Preview width': 'Szerokość podglądu', 'Preview opacity': 'Przezroczystość podglądu',
    'Show / hide sidebar': 'Pokaż / ukryj panel', 'Open settings': 'Otwórz ustawienia',
    'Click a field and press a new combination (with Ctrl, Alt or Cmd).': 'Kliknij pole i naciśnij nową kombinację (z Ctrl, Alt lub Cmd).',
    'Productive today': 'Produktywnie dziś', 'Productive this week': 'Produktywnie w tym tygodniu',
    'Sums timed events from calendars marked "Counts as productive". Click a number to copy.': 'Sumuje wydarzenia z godzinami z kalendarzy „Liczy się jako produktywny”. Kliknij liczbę, aby skopiować.',
    'SideCal by Kacper Górec · open source under the Apache License 2.0': 'SideCal – Kacper Górec · open source na licencji Apache 2.0',
    'Use the calendar’s own colour': 'Użyj własnego koloru kalendarza', 'Change calendar colour': 'Zmień kolor kalendarza', 'Counts as productive': 'Liczy się jako produktywny',
    'Launch at Login': 'Uruchamiaj przy logowaniu', 'Display All Day Events': 'Pokazuj wydarzenia całodniowe',
    'Keep Above Other Windows': 'Trzymaj nad innymi oknami', 'Keeps the sidebar on top of other windows. Turn it off to let other windows cover it.': 'Utrzymuje panel nad innymi oknami. Wyłącz, aby inne okna mogły go zasłaniać.',
    'Auto-Hide': 'Autoukrywanie', 'Slides the sidebar to the screen edge when the cursor leaves it, and back out when you touch the edge.': 'Chowa panel do krawędzi ekranu, gdy kursor go opuści, i wysuwa po dotknięciu krawędzi.',
    'Island Mode': 'Tryb wyspy', 'Shows the open sidebar as a floating card: transparent margin around it and rounded corners, like a macOS window.': 'Pokazuje otwarty panel jako unoszącą się kartę: przezroczysty margines i zaokrąglone rogi, jak okno macOS.',
    'Highlight Active Event': 'Wyróżniaj trwające wydarzenie', 'Marks the event happening right now with a ring around its entry. In the edge preview it is drawn at full opacity while the other entries stay faded.': 'Zaznacza wydarzenie trwające teraz obwódką wokół wpisu. W podglądzie na krawędzi jest w pełni widoczne, a pozostałe wpisy pozostają wyblakłe.',
    'Suggest Events': 'Podpowiadaj wydarzenia', 'When you add an event with the + button or the shortcut, it starts in the next free half-hour slot and lasts an hour, instead of starting right now.': 'Wydarzenie dodane przyciskiem + lub skrótem zaczyna się w najbliższym wolnym półgodzinnym oknie i trwa godzinę, zamiast zaczynać się od razu.',
    'Show Event Duration in Menu Bar': 'Pokazuj czas wydarzenia na pasku menu', 'Show Event Title in Menu Bar': 'Pokazuj tytuł wydarzenia na pasku menu',
    'Use 12-Hour Time Format': 'Format 12-godzinny', 'Shows 1-12 instead of 0-23 (no AM/PM).': 'Pokazuje 1–12 zamiast 0–23 (bez AM/PM).',
    'Zoom Timeline with Scroll': 'Powiększaj oś czasu przewijaniem', 'Hold Ctrl or Cmd and scroll (or pinch on a trackpad) to zoom the timeline in. Plain scroll then moves up and down; zooming fully out fits the whole day again.':
      'Przytrzymaj Ctrl lub Cmd i przewijaj (lub użyj gestu szczypania na gładziku), aby powiększyć oś czasu. Zwykłe przewijanie przesuwa w górę i w dół; pełne oddalenie znów mieści cały dzień.',
    'Show Zoom Mini-Map': 'Pokazuj minimapę zoomu',
    'While zoomed in, a thin overview of the whole day appears on the edge of the panel, with a frame around what you see.': 'Przy powiększeniu na krawędzi panelu pojawia się cienki podgląd całego dnia z ramką wokół tego, co widzisz.',
    'Show Time Indicator': 'Pokazuj wskaźnik czasu', 'Show Header': 'Pokazuj nagłówek',
    'Hides the whole header: weekday, date and the settings button. Right-click the board for Settings.': 'Ukrywa cały nagłówek: dzień tygodnia, datę i przycisk ustawień. Ustawienia: kliknij panel prawym przyciskiem.',
    'Show Weekday': 'Pokazuj dzień tygodnia', 'Show Date': 'Pokazuj datę', 'Use Global Hotkeys': 'Globalne skróty',
    'Quit SideCal': 'Zakończ SideCal', 'Starting now': 'Zaczyna się teraz', 'Starts in {n} min': 'Zaczyna się za {n} min',
  },
  de: {
    'color:red': 'Rot', 'color:coral': 'Koralle', 'color:orange': 'Orange', 'color:yellow': 'Gelb', 'color:lime': 'Limette', 'color:green': 'Grün', 'color:teal': 'Türkis', 'color:blue': 'Blau', 'color:purple': 'Violett', 'color:pink': 'Rosa', 'color:brown': 'Braun', 'color:gray': 'Grau',
    'Settings': 'Einstellungen', 'Settings…': 'Einstellungen…', 'New event': 'Neuer Termin', 'Event title': 'Titel des Termins',
    'Cancel': 'Abbrechen', 'Save': 'Sichern', 'Reload': 'Neu laden', 'Untitled': 'Ohne Titel',
    'Waiting for calendar access…': 'Warte auf Kalenderzugriff…',
    'System Settings → Privacy & Security → Calendars': 'Systemeinstellungen → Datenschutz & Sicherheit → Kalender',
    'Show hidden events': 'Ausgeblendete Termine zeigen', 'Hide hidden events': 'Ausgeblendete Termine verbergen',
    'Use calendar colour': 'Kalenderfarbe verwenden', 'Unhide event': 'Termin einblenden', 'Hide event': 'Termin ausblenden',
    'Open in Calendar': 'In Kalender öffnen', 'Delete event': 'Termin löschen', 'Click again to delete': 'Zum Löschen erneut klicken',
    'End must be after start': 'Das Ende muss nach dem Beginn liegen',
    'Calendar': 'Kalender', 'No more events': 'Keine weiteren Termine', '{t} left': 'noch {t}', 'in {t}': 'in {t}', 'Next in {t}': 'Nächster in {t}',
    'No reminder': 'Keine Erinnerung', 'At time of event': 'Zum Zeitpunkt des Termins', '{n} min before': '{n} Min. vorher',
    '{n} hour before': '{n} Stunde vorher', '{n} hours before': '{n} Stunden vorher', '{n} day before': '{n} Tag vorher', '{n} days before': '{n} Tage vorher',
    'Off': 'Aus', 'At start': 'Zum Beginn',
    'Glance at your day without breaking focus.': 'Wirf einen Blick auf deinen Tag, ohne die Arbeit zu unterbrechen.',
    'Calendars': 'Kalender', 'General': 'Allgemein', 'Appearance': 'Darstellung', 'Sidebar': 'Seitenleiste', 'Shortcuts': 'Tastenkürzel', 'Productivity': 'Produktivität',
    'Untick the swatch to hide a calendar. Drag the handle to reorder: higher = priority.': 'Farbfeld abwählen, um einen Kalender auszublenden. Am Griff ziehen zum Sortieren: weiter oben = höhere Priorität.',
    'Startup': 'Start', 'Menu bar': 'Menüleiste', 'New events': 'Neue Termine', 'Reminders': 'Erinnerungen', 'Default reminder': 'Standard-Erinnerung',
    'Preselected for new events and saved to the calendar. "Off" means no reminder there either.': 'Für neue Termine vorausgewählt und im Kalender gespeichert. „Aus“ heißt auch dort keine Erinnerung.',
    'Theme': 'Design', 'Light': 'Hell', 'Dark': 'Dunkel', 'Auto': 'Auto', 'Follow the macOS appearance': 'Dem macOS-Erscheinungsbild folgen',
    'Event style': 'Terminstil', 'Solid': 'Voll', 'Tinted': 'Getönt', 'Filled with the event colour': 'Mit der Terminfarbe gefüllt',
    'A soft tint with a colour bar on the left': 'Ein zarter Farbton mit Farbbalken links',
    'Header & layout': 'Kopfzeile & Layout', 'Timeline': 'Zeitleiste', 'Position & behaviour': 'Position & Verhalten',
    'Sidebar side': 'Seite der Leiste', 'Left': 'Links', 'Right': 'Rechts', 'Sidebar width': 'Breite der Leiste', 'Reveal delay': 'Einblendverzögerung',
    'Edge preview': 'Randvorschau', 'Leave faded entries on the screen edge while hidden': 'Verblasste Einträge am Bildschirmrand lassen, solange die Leiste verborgen ist',
    'Preview width': 'Vorschaubreite', 'Preview opacity': 'Vorschau-Deckkraft',
    'Show / hide sidebar': 'Seitenleiste ein-/ausblenden', 'Open settings': 'Einstellungen öffnen',
    'Click a field and press a new combination (with Ctrl, Alt or Cmd).': 'Feld anklicken und neue Kombination drücken (mit Ctrl, Alt oder Cmd).',
    'Productive today': 'Heute produktiv', 'Productive this week': 'Diese Woche produktiv',
    'Sums timed events from calendars marked "Counts as productive". Click a number to copy.': 'Summiert Termine mit Uhrzeit aus Kalendern mit „Zählt als produktiv“. Zahl anklicken zum Kopieren.',
    'SideCal by Kacper Górec · open source under the Apache License 2.0': 'SideCal von Kacper Górec · Open Source unter der Apache License 2.0',
    'Use the calendar’s own colour': 'Eigene Farbe des Kalenders verwenden', 'Change calendar colour': 'Kalenderfarbe ändern', 'Counts as productive': 'Zählt als produktiv',
    'Launch at Login': 'Beim Anmelden starten', 'Display All Day Events': 'Ganztägige Termine anzeigen',
    'Keep Above Other Windows': 'Über anderen Fenstern halten', 'Keeps the sidebar on top of other windows. Turn it off to let other windows cover it.': 'Hält die Seitenleiste über anderen Fenstern. Ausschalten, damit andere Fenster sie verdecken können.',
    'Auto-Hide': 'Automatisch ausblenden', 'Slides the sidebar to the screen edge when the cursor leaves it, and back out when you touch the edge.': 'Schiebt die Seitenleiste an den Bildschirmrand, wenn der Zeiger sie verlässt, und wieder heraus, wenn du den Rand berührst.',
    'Island Mode': 'Insel-Modus', 'Shows the open sidebar as a floating card: transparent margin around it and rounded corners, like a macOS window.': 'Zeigt die geöffnete Seitenleiste als schwebende Karte: transparenter Rand und abgerundete Ecken wie ein macOS-Fenster.',
    'Highlight Active Event': 'Aktuellen Termin hervorheben', 'Marks the event happening right now with a ring around its entry. In the edge preview it is drawn at full opacity while the other entries stay faded.': 'Markiert den laufenden Termin mit einem Ring um den Eintrag. In der Randvorschau wird er voll deckend gezeichnet, während die übrigen Einträge verblasst bleiben.',
    'Suggest Events': 'Termine vorschlagen', 'When you add an event with the + button or the shortcut, it starts in the next free half-hour slot and lasts an hour, instead of starting right now.': 'Ein mit der +-Taste oder dem Kürzel angelegter Termin beginnt im nächsten freien Halbstundenfenster und dauert eine Stunde, statt sofort zu beginnen.',
    'Show Event Duration in Menu Bar': 'Termindauer in der Menüleiste zeigen', 'Show Event Title in Menu Bar': 'Termintitel in der Menüleiste zeigen',
    'Use 12-Hour Time Format': '12-Stunden-Format verwenden', 'Shows 1-12 instead of 0-23 (no AM/PM).': 'Zeigt 1–12 statt 0–23 (ohne AM/PM).',
    'Zoom Timeline with Scroll': 'Zeitleiste per Scrollen zoomen', 'Hold Ctrl or Cmd and scroll (or pinch on a trackpad) to zoom the timeline in. Plain scroll then moves up and down; zooming fully out fits the whole day again.':
      'Ctrl oder Cmd halten und scrollen (oder auf dem Trackpad zusammenziehen/spreizen), um die Zeitleiste zu zoomen. Normales Scrollen bewegt dann nach oben und unten; ganz herauszoomen zeigt wieder den ganzen Tag.',
    'Show Zoom Mini-Map': 'Zoom-Minikarte zeigen',
    'While zoomed in, a thin overview of the whole day appears on the edge of the panel, with a frame around what you see.': 'Beim Hineinzoomen erscheint am Rand der Leiste eine schmale Übersicht des ganzen Tages mit einem Rahmen um den sichtbaren Ausschnitt.',
    'Show Time Indicator': 'Zeitanzeige zeigen', 'Show Header': 'Kopfzeile zeigen',
    'Hides the whole header: weekday, date and the settings button. Right-click the board for Settings.': 'Blendet die ganze Kopfzeile aus: Wochentag, Datum und die Einstellungen-Taste. Einstellungen per Rechtsklick auf die Leiste.',
    'Show Weekday': 'Wochentag zeigen', 'Show Date': 'Datum zeigen', 'Use Global Hotkeys': 'Globale Tastenkürzel verwenden',
    'Quit SideCal': 'SideCal beenden', 'Starting now': 'Beginnt jetzt', 'Starts in {n} min': 'Beginnt in {n} Min.',
  },
  es: {
    'color:red': 'Rojo', 'color:coral': 'Coral', 'color:orange': 'Naranja', 'color:yellow': 'Amarillo', 'color:lime': 'Lima', 'color:green': 'Verde', 'color:teal': 'Turquesa', 'color:blue': 'Azul', 'color:purple': 'Morado', 'color:pink': 'Rosa', 'color:brown': 'Marrón', 'color:gray': 'Gris',
    'Settings': 'Ajustes', 'Settings…': 'Ajustes…', 'New event': 'Nuevo evento', 'Event title': 'Título del evento',
    'Cancel': 'Cancelar', 'Save': 'Guardar', 'Reload': 'Recargar', 'Untitled': 'Sin título',
    'Waiting for calendar access…': 'Esperando acceso al calendario…',
    'System Settings → Privacy & Security → Calendars': 'Ajustes del Sistema → Privacidad y seguridad → Calendarios',
    'Show hidden events': 'Mostrar eventos ocultos', 'Hide hidden events': 'Ocultar eventos ocultos',
    'Use calendar colour': 'Usar el color del calendario', 'Unhide event': 'Mostrar evento', 'Hide event': 'Ocultar evento',
    'Open in Calendar': 'Abrir en Calendario', 'Delete event': 'Eliminar evento', 'Click again to delete': 'Haz clic de nuevo para eliminar',
    'End must be after start': 'El fin debe ser posterior al inicio',
    'Calendar': 'Calendario', 'No more events': 'No hay más eventos', '{t} left': 'quedan {t}', 'in {t}': 'en {t}', 'Next in {t}': 'Siguiente en {t}',
    'No reminder': 'Sin recordatorio', 'At time of event': 'A la hora del evento', '{n} min before': '{n} min antes',
    '{n} hour before': '{n} hora antes', '{n} hours before': '{n} horas antes', '{n} day before': '{n} día antes', '{n} days before': '{n} días antes',
    'Off': 'No', 'At start': 'Al inicio',
    'Glance at your day without breaking focus.': 'Echa un vistazo a tu día sin dejar de trabajar.',
    'Calendars': 'Calendarios', 'General': 'General', 'Appearance': 'Apariencia', 'Sidebar': 'Barra lateral', 'Shortcuts': 'Atajos', 'Productivity': 'Productividad',
    'Untick the swatch to hide a calendar. Drag the handle to reorder: higher = priority.': 'Desmarca la muestra para ocultar un calendario. Arrastra el asa para reordenar: más arriba = más prioridad.',
    'Startup': 'Inicio', 'Menu bar': 'Barra de menús', 'New events': 'Nuevos eventos', 'Reminders': 'Recordatorios', 'Default reminder': 'Recordatorio predeterminado',
    'Preselected for new events and saved to the calendar. "Off" means no reminder there either.': 'Preseleccionado en eventos nuevos y guardado en el calendario. «No» significa sin recordatorio también allí.',
    'Theme': 'Tema', 'Light': 'Claro', 'Dark': 'Oscuro', 'Auto': 'Auto', 'Follow the macOS appearance': 'Seguir la apariencia de macOS',
    'Event style': 'Estilo de eventos', 'Solid': 'Sólido', 'Tinted': 'Teñido', 'Filled with the event colour': 'Relleno con el color del evento',
    'A soft tint with a colour bar on the left': 'Un tono suave con una barra de color a la izquierda',
    'Header & layout': 'Encabezado y diseño', 'Timeline': 'Línea de tiempo', 'Position & behaviour': 'Posición y comportamiento',
    'Sidebar side': 'Lado de la barra', 'Left': 'Izquierda', 'Right': 'Derecha', 'Sidebar width': 'Ancho de la barra', 'Reveal delay': 'Retraso al mostrar',
    'Edge preview': 'Vista previa en el borde', 'Leave faded entries on the screen edge while hidden': 'Dejar entradas atenuadas en el borde de la pantalla mientras está oculta',
    'Preview width': 'Ancho de la vista previa', 'Preview opacity': 'Opacidad de la vista previa',
    'Show / hide sidebar': 'Mostrar / ocultar la barra', 'Open settings': 'Abrir ajustes',
    'Click a field and press a new combination (with Ctrl, Alt or Cmd).': 'Haz clic en un campo y pulsa una nueva combinación (con Ctrl, Alt o Cmd).',
    'Productive today': 'Productivo hoy', 'Productive this week': 'Productivo esta semana',
    'Sums timed events from calendars marked "Counts as productive". Click a number to copy.': 'Suma los eventos con hora de calendarios «Cuenta como productivo». Haz clic en un número para copiarlo.',
    'SideCal by Kacper Górec · open source under the Apache License 2.0': 'SideCal de Kacper Górec · código abierto bajo la licencia Apache 2.0',
    'Use the calendar’s own colour': 'Usar el color propio del calendario', 'Change calendar colour': 'Cambiar el color del calendario', 'Counts as productive': 'Cuenta como productivo',
    'Launch at Login': 'Abrir al iniciar sesión', 'Display All Day Events': 'Mostrar eventos de todo el día',
    'Keep Above Other Windows': 'Mantener sobre otras ventanas', 'Keeps the sidebar on top of other windows. Turn it off to let other windows cover it.': 'Mantiene la barra por encima de otras ventanas. Desactívalo para que otras ventanas puedan cubrirla.',
    'Auto-Hide': 'Ocultar automáticamente', 'Slides the sidebar to the screen edge when the cursor leaves it, and back out when you touch the edge.': 'Desliza la barra al borde de la pantalla cuando el cursor la abandona y la saca al tocar el borde.',
    'Island Mode': 'Modo isla', 'Shows the open sidebar as a floating card: transparent margin around it and rounded corners, like a macOS window.': 'Muestra la barra abierta como una tarjeta flotante: margen transparente y esquinas redondeadas, como una ventana de macOS.',
    'Highlight Active Event': 'Resaltar el evento actual', 'Marks the event happening right now with a ring around its entry. In the edge preview it is drawn at full opacity while the other entries stay faded.': 'Marca el evento en curso con un aro alrededor de su entrada. En la vista previa del borde se dibuja con plena opacidad mientras las demás entradas siguen atenuadas.',
    'Suggest Events': 'Sugerir eventos', 'When you add an event with the + button or the shortcut, it starts in the next free half-hour slot and lasts an hour, instead of starting right now.': 'Al añadir un evento con el botón + o el atajo, empieza en el siguiente hueco libre de media hora y dura una hora, en lugar de empezar ahora mismo.',
    'Show Event Duration in Menu Bar': 'Mostrar duración del evento en la barra de menús', 'Show Event Title in Menu Bar': 'Mostrar título del evento en la barra de menús',
    'Use 12-Hour Time Format': 'Usar formato de 12 horas', 'Shows 1-12 instead of 0-23 (no AM/PM).': 'Muestra 1–12 en lugar de 0–23 (sin AM/PM).',
    'Zoom Timeline with Scroll': 'Zoom de la línea de tiempo con scroll', 'Hold Ctrl or Cmd and scroll (or pinch on a trackpad) to zoom the timeline in. Plain scroll then moves up and down; zooming fully out fits the whole day again.':
      'Mantén Ctrl o Cmd y desplázate (o pellizca en el trackpad) para ampliar la línea de tiempo. El scroll normal sube y baja; al alejar del todo vuelve a caber el día completo.',
    'Show Zoom Mini-Map': 'Mostrar minimapa del zoom',
    'While zoomed in, a thin overview of the whole day appears on the edge of the panel, with a frame around what you see.': 'Al ampliar, aparece en el borde del panel una vista fina de todo el día con un marco alrededor de lo que ves.',
    'Show Time Indicator': 'Mostrar indicador de hora', 'Show Header': 'Mostrar encabezado',
    'Hides the whole header: weekday, date and the settings button. Right-click the board for Settings.': 'Oculta todo el encabezado: día de la semana, fecha y el botón de ajustes. Haz clic derecho en el panel para Ajustes.',
    'Show Weekday': 'Mostrar día de la semana', 'Show Date': 'Mostrar fecha', 'Use Global Hotkeys': 'Usar atajos globales',
    'Quit SideCal': 'Salir de SideCal', 'Starting now': 'Empieza ahora', 'Starts in {n} min': 'Empieza en {n} min',
  },
  fr: {
    'color:red': 'Rouge', 'color:coral': 'Corail', 'color:orange': 'Orange', 'color:yellow': 'Jaune', 'color:lime': 'Citron vert', 'color:green': 'Vert', 'color:teal': 'Sarcelle', 'color:blue': 'Bleu', 'color:purple': 'Violet', 'color:pink': 'Rose', 'color:brown': 'Marron', 'color:gray': 'Gris',
    'Settings': 'Réglages', 'Settings…': 'Réglages…', 'New event': 'Nouvel événement', 'Event title': 'Titre de l’événement',
    'Cancel': 'Annuler', 'Save': 'Enregistrer', 'Reload': 'Recharger', 'Untitled': 'Sans titre',
    'Waiting for calendar access…': 'En attente de l’accès au calendrier…',
    'System Settings → Privacy & Security → Calendars': 'Réglages Système → Confidentialité et sécurité → Calendriers',
    'Show hidden events': 'Afficher les événements masqués', 'Hide hidden events': 'Masquer les événements masqués',
    'Use calendar colour': 'Utiliser la couleur du calendrier', 'Unhide event': 'Afficher l’événement', 'Hide event': 'Masquer l’événement',
    'Open in Calendar': 'Ouvrir dans Calendrier', 'Delete event': 'Supprimer l’événement', 'Click again to delete': 'Cliquez à nouveau pour supprimer',
    'End must be after start': 'La fin doit être après le début',
    'Calendar': 'Calendrier', 'No more events': 'Plus d’événements', '{t} left': '{t} restant', 'in {t}': 'dans {t}', 'Next in {t}': 'Prochain dans {t}',
    'No reminder': 'Aucun rappel', 'At time of event': 'À l’heure de l’événement', '{n} min before': '{n} min avant',
    '{n} hour before': '{n} heure avant', '{n} hours before': '{n} heures avant', '{n} day before': '{n} jour avant', '{n} days before': '{n} jours avant',
    'Off': 'Non', 'At start': 'Au début',
    'Glance at your day without breaking focus.': 'Jetez un œil à votre journée sans quitter votre travail.',
    'Calendars': 'Calendriers', 'General': 'Général', 'Appearance': 'Apparence', 'Sidebar': 'Barre latérale', 'Shortcuts': 'Raccourcis', 'Productivity': 'Productivité',
    'Untick the swatch to hide a calendar. Drag the handle to reorder: higher = priority.': 'Décochez la pastille pour masquer un calendrier. Faites glisser la poignée pour réordonner : plus haut = prioritaire.',
    'Startup': 'Démarrage', 'Menu bar': 'Barre des menus', 'New events': 'Nouveaux événements', 'Reminders': 'Rappels', 'Default reminder': 'Rappel par défaut',
    'Preselected for new events and saved to the calendar. "Off" means no reminder there either.': 'Présélectionné pour les nouveaux événements et enregistré dans le calendrier. « Non » = aucun rappel là-bas non plus.',
    'Theme': 'Thème', 'Light': 'Clair', 'Dark': 'Sombre', 'Auto': 'Auto', 'Follow the macOS appearance': 'Suivre l’apparence de macOS',
    'Event style': 'Style des événements', 'Solid': 'Plein', 'Tinted': 'Teinté', 'Filled with the event colour': 'Rempli avec la couleur de l’événement',
    'A soft tint with a colour bar on the left': 'Une teinte douce avec une barre de couleur à gauche',
    'Header & layout': 'En-tête et mise en page', 'Timeline': 'Chronologie', 'Position & behaviour': 'Position et comportement',
    'Sidebar side': 'Côté de la barre', 'Left': 'Gauche', 'Right': 'Droite', 'Sidebar width': 'Largeur de la barre', 'Reveal delay': 'Délai d’apparition',
    'Edge preview': 'Aperçu sur le bord', 'Leave faded entries on the screen edge while hidden': 'Laisser des entrées estompées sur le bord de l’écran quand elle est masquée',
    'Preview width': 'Largeur de l’aperçu', 'Preview opacity': 'Opacité de l’aperçu',
    'Show / hide sidebar': 'Afficher / masquer la barre', 'Open settings': 'Ouvrir les réglages',
    'Click a field and press a new combination (with Ctrl, Alt or Cmd).': 'Cliquez sur un champ et appuyez sur une nouvelle combinaison (avec Ctrl, Alt ou Cmd).',
    'Productive today': 'Productif aujourd’hui', 'Productive this week': 'Productif cette semaine',
    'Sums timed events from calendars marked "Counts as productive". Click a number to copy.': 'Additionne les événements horaires des calendriers « Compte comme productif ». Cliquez sur un nombre pour le copier.',
    'SideCal by Kacper Górec · open source under the Apache License 2.0': 'SideCal par Kacper Górec · open source sous licence Apache 2.0',
    'Use the calendar’s own colour': 'Utiliser la couleur propre du calendrier', 'Change calendar colour': 'Changer la couleur du calendrier', 'Counts as productive': 'Compte comme productif',
    'Launch at Login': 'Ouvrir à la connexion', 'Display All Day Events': 'Afficher les événements de la journée',
    'Keep Above Other Windows': 'Garder au-dessus des autres fenêtres', 'Keeps the sidebar on top of other windows. Turn it off to let other windows cover it.': 'Garde la barre au-dessus des autres fenêtres. Désactivez pour que d’autres fenêtres puissent la recouvrir.',
    'Auto-Hide': 'Masquage automatique', 'Slides the sidebar to the screen edge when the cursor leaves it, and back out when you touch the edge.': 'Fait glisser la barre vers le bord de l’écran quand le curseur la quitte, et la ressort quand vous touchez le bord.',
    'Island Mode': 'Mode îlot', 'Shows the open sidebar as a floating card: transparent margin around it and rounded corners, like a macOS window.': 'Affiche la barre ouverte comme une carte flottante : marge transparente et coins arrondis, comme une fenêtre macOS.',
    'Highlight Active Event': 'Mettre en évidence l’événement en cours', 'Marks the event happening right now with a ring around its entry. In the edge preview it is drawn at full opacity while the other entries stay faded.': 'Signale l’événement en cours avec un anneau autour de son entrée. Dans l’aperçu sur le bord, il est affiché en pleine opacité alors que les autres entrées restent estompées.',
    'Suggest Events': 'Suggérer des événements', 'When you add an event with the + button or the shortcut, it starts in the next free half-hour slot and lasts an hour, instead of starting right now.': 'Un événement ajouté avec le bouton + ou le raccourci commence au prochain créneau libre d’une demi-heure et dure une heure, au lieu de commencer tout de suite.',
    'Show Event Duration in Menu Bar': 'Afficher la durée de l’événement dans la barre des menus', 'Show Event Title in Menu Bar': 'Afficher le titre de l’événement dans la barre des menus',
    'Use 12-Hour Time Format': 'Format 12 heures', 'Shows 1-12 instead of 0-23 (no AM/PM).': 'Affiche 1–12 au lieu de 0–23 (sans AM/PM).',
    'Zoom Timeline with Scroll': 'Zoomer la chronologie au défilement', 'Hold Ctrl or Cmd and scroll (or pinch on a trackpad) to zoom the timeline in. Plain scroll then moves up and down; zooming fully out fits the whole day again.':
      'Maintenez Ctrl ou Cmd et faites défiler (ou pincez sur un trackpad) pour zoomer la chronologie. Le défilement simple monte et descend ; un dézoom complet affiche de nouveau toute la journée.',
    'Show Zoom Mini-Map': 'Afficher la mini-carte du zoom',
    'While zoomed in, a thin overview of the whole day appears on the edge of the panel, with a frame around what you see.': 'Lors d’un zoom avant, un fin aperçu de toute la journée apparaît sur le bord du panneau, avec un cadre autour de ce que vous voyez.',
    'Show Time Indicator': 'Afficher l’indicateur d’heure', 'Show Header': 'Afficher l’en-tête',
    'Hides the whole header: weekday, date and the settings button. Right-click the board for Settings.': 'Masque tout l’en-tête : jour de la semaine, date et bouton réglages. Clic droit sur la barre pour les Réglages.',
    'Show Weekday': 'Afficher le jour de la semaine', 'Show Date': 'Afficher la date', 'Use Global Hotkeys': 'Raccourcis globaux',
    'Quit SideCal': 'Quitter SideCal', 'Starting now': 'Commence maintenant', 'Starts in {n} min': 'Commence dans {n} min',
  },
};

const LANGS = ['en', 'pl', 'de', 'es', 'fr'];
const LANG = (() => {
  try { const v = localStorage.getItem('lang'); if (LANGS.includes(v)) return v; } catch {}
  for (const l of navigator.languages || [navigator.language]) {
    const c = String(l).slice(0, 2).toLowerCase();
    if (c === 'en') return 'en';
    if (I18N[c]) return c;
  }
  return 'en';
})();
const DICT = I18N[LANG] || {};

// Palette colour names for swatch tooltips (falls back to the capitalised id).
const colorName = (id) => DICT['color:' + id] || id.charAt(0).toUpperCase() + id.slice(1);

function t(s, vars) {
  let r = DICT[s] || s;
  if (vars) for (const k in vars) r = r.replace('{' + k + '}', vars[k]);
  return r;
}

// Translates static markup: text nodes and title / aria-label / placeholder attributes under `root`.
function applyI18n(root = document.body) {
  if (LANG === 'en') return;
  const w = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
  for (let n = w.nextNode(); n; n = w.nextNode()) {
    if (n.parentNode.closest('script,style')) continue;
    const s = n.nodeValue.trim();
    if (DICT[s]) n.nodeValue = n.nodeValue.replace(s, DICT[s]);
  }
  for (const el of [root, ...root.querySelectorAll('[title],[aria-label],[placeholder]')]) {
    for (const a of ['title', 'aria-label', 'placeholder']) {
      const v = el.getAttribute && el.getAttribute(a);
      if (v && DICT[v]) el.setAttribute(a, DICT[v]);
    }
  }
}
document.documentElement.lang = LANG;

// Another SideCal window changed the language: reload so every window follows.
window.addEventListener('storage', (e) => { if (e.key === 'lang') location.reload(); });

// Round flags (inline SVG, so nothing is cropped) in a dropdown: button shows the current language.
const FLAG = {
  en: '<rect width="24" height="24" fill="#1d3b8b"/><path d="M0 0L24 24M24 0L0 24" stroke="#fff" stroke-width="5"/><path d="M0 0L24 24M24 0L0 24" stroke="#c8202f" stroke-width="2"/><path d="M12 0v24M0 12h24" stroke="#fff" stroke-width="8"/><path d="M12 0v24M0 12h24" stroke="#c8202f" stroke-width="4.5"/>',
  pl: '<rect width="24" height="12" fill="#f4f4f4"/><rect y="12" width="24" height="12" fill="#d4213d"/>',
  de: '<rect width="24" height="8" fill="#1a1a1a"/><rect y="8" width="24" height="8" fill="#dd0000"/><rect y="16" width="24" height="8" fill="#ffce00"/>',
  es: '<rect width="24" height="24" fill="#c60b1e"/><rect y="6" width="24" height="12" fill="#ffc400"/>',
  fr: '<rect width="8" height="24" fill="#1f3f9a"/><rect x="8" width="8" height="24" fill="#f4f4f4"/><rect x="16" width="8" height="24" fill="#e1262f"/>',
};
const flagSvg = (l, px) => `<svg width="${px}" height="${px}" viewBox="0 0 24 24"><clipPath id="c${l}${px}"><circle cx="12" cy="12" r="12"/></clipPath><g clip-path="url(#c${l}${px})">${FLAG[l]}</g></svg>`;
function langPicker() {
  const NAMES = { en: 'English', pl: 'Polski', de: 'Deutsch', es: 'Español', fr: 'Français' };
  const box = document.createElement('div');
  box.className = 'langs';
  const btn = document.createElement('button');
  btn.type = 'button'; btn.className = 'cur'; btn.title = NAMES[LANG]; btn.innerHTML = flagSvg(LANG, 22);
  const list = document.createElement('div');
  list.className = 'menu'; list.hidden = true;
  for (const l of LANGS) {
    const b = document.createElement('button');
    b.type = 'button'; b.className = l === LANG ? 'on' : '';
    b.innerHTML = `${flagSvg(l, 18)}<span>${NAMES[l]}</span>`;
    b.onclick = () => { try { localStorage.setItem('lang', l); } catch {} location.reload(); };
    list.appendChild(b);
  }
  btn.onclick = (e) => { e.stopPropagation(); list.hidden = !list.hidden; };
  document.addEventListener('click', () => { list.hidden = true; });
  box.append(btn, list);
  return box;
}
