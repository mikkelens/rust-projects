use ical::generator::{Emitter, IcalCalendar, IcalEvent};
use ical::parser::ical::component::{
    IcalAlarm, IcalFreeBusy, IcalJournal, IcalTimeZone, IcalTimeZoneTransition, IcalTodo,
};
use ical::property::Property;

pub(crate) trait FilterBuild {
    fn build(&self) -> String;
}
impl FilterBuild for IcalCalendar {
    fn build(&self) -> String {
        let new = IcalCalendar {
            properties: self.properties.clone_filtered(),
            events: self.events.clone_filtered(),
            alarms: self.alarms.clone_filtered(),
            todos: self.todos.clone_filtered(),
            journals: self.journals.clone_filtered(),
            free_busys: self.free_busys.clone_filtered(),
            timezones: self.timezones.clone_filtered(),
        };
        new.generate()
    }
}

trait PropFiltered {
    fn prop_filter(property: &&Property) -> bool {
        property
            .value
            .as_ref()
            .is_some_and(|s| s.as_str() == "Beregnelighed og logik")
            && property.name == "SUMMARY"
    }
    fn clone_filtered(&self) -> Self;
}
impl PropFiltered for Vec<Property> {
    fn clone_filtered(&self) -> Self {
        self.iter().filter(Self::prop_filter).cloned().collect()
    }
}
impl PropFiltered for Vec<IcalEvent> {
    fn clone_filtered(&self) -> Self {
        self.iter()
            .map(|event| IcalEvent {
                properties: event.properties.clone_filtered(),
                alarms: event.alarms.clone_filtered(),
            })
            .collect()
    }
}
impl PropFiltered for Vec<IcalAlarm> {
    fn clone_filtered(&self) -> Self {
        self.iter()
            .map(|alarm| IcalAlarm {
                properties: alarm.properties.clone_filtered(),
            })
            .collect()
    }
}
impl PropFiltered for Vec<IcalTodo> {
    fn clone_filtered(&self) -> Self {
        self.iter()
            .map(|todo| IcalTodo {
                alarms: todo.alarms.clone_filtered(),
                properties: todo.properties.clone_filtered(),
            })
            .collect()
    }
}
impl PropFiltered for Vec<IcalJournal> {
    fn clone_filtered(&self) -> Self {
        self.iter()
            .map(|journal| IcalJournal {
                properties: journal.properties.clone_filtered(),
            })
            .collect()
    }
}
impl PropFiltered for Vec<IcalFreeBusy> {
    fn clone_filtered(&self) -> Self {
        self.iter()
            .map(|free_busy| IcalFreeBusy {
                properties: free_busy.properties.clone_filtered(),
            })
            .collect()
    }
}
impl PropFiltered for Vec<IcalTimeZone> {
    fn clone_filtered(&self) -> Self {
        self.iter()
            .map(|timezone| IcalTimeZone {
                transitions: timezone
                    .transitions
                    .iter()
                    .map(|transition| IcalTimeZoneTransition {
                        transition: transition.transition.clone(),
                        properties: transition.properties.clone_filtered(),
                    })
                    .collect(),
                properties: timezone.properties.clone_filtered(),
            })
            .collect()
    }
}
