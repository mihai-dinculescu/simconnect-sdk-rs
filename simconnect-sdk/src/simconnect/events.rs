use crate::{
    bindings, success, ClientEvent, NotificationGroup, SimConnect, SimConnectError,
    SystemEventRequest,
};

impl SimConnect {
    /// Associates a client defined event with a Microsoft Flight Simulator event name.
    ///
    /// WIP
    #[tracing::instrument(name = "SimConnect::register_event", level = "debug", skip(self))]
    pub fn register_event(
        &self,
        event: ClientEvent,
        notification_group: NotificationGroup,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        })?;

        success!(unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                notification_group as u32,
                event as u32,
                0,
            )
        })?;

        success!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(
                self.handle.as_ptr(),
                notification_group as u32,
                1,
            )
        })
    }

    /// Request that a specific system event is notified to the client.
    #[tracing::instrument(
        name = "SimConnect::subscribe_to_system_event",
        level = "debug",
        skip(self)
    )]
    pub fn subscribe_to_system_event(
        &mut self,
        event: SystemEventRequest,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_SubscribeToSystemEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        })
    }

    /// Request that notifications are no longer received for the specified system event.
    #[tracing::instrument(
        name = "SimConnect::unsubscribe_from_system_event",
        level = "debug",
        skip(self)
    )]
    pub fn unsubscribe_from_system_event(
        &mut self,
        event: SystemEventRequest,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_UnsubscribeFromSystemEvent(self.handle.as_ptr(), event as u32)
        })
    }
}
