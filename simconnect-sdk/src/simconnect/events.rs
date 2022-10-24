use crate::{bindings, success, Event, NotificationGroup, SimConnect, SimConnectError};

impl SimConnect {
    /// Associates a client defined event with a Microsoft Flight Simulator event name.
    ///
    /// WIP
    #[tracing::instrument(name = "SimConnect::register_event", level = "debug", skip(self))]
    pub fn register_event(
        &self,
        event: Event,
        notification_group: NotificationGroup,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        });

        success!(unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                notification_group as u32,
                event as u32,
                0,
            )
        });

        success!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(
                self.handle.as_ptr(),
                notification_group as u32,
                1,
            )
        });

        Ok(())
    }
}
