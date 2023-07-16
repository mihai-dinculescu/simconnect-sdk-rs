use crate::{
    bindings, success, ClientEvent, EventFlag, SimConnect, SimConnectError, SystemEventRequest, ClientEventRequest
};

// In order to simplify the usage we're using a single notification group for all client events.
const NOTIFICATION_GROUP_ID: u32 = 0;

impl SimConnect {
    /// Request that a specific system event is notified.
    #[tracing::instrument(
        name = "SimConnect::subscribe_to_system_event",
        level = "debug",
        skip(self)
    )]
    pub fn subscribe_to_system_event(
        &mut self,
        event: SystemEventRequest,
    ) -> Result<(), SimConnectError> {
        self.system_event_register.register(event)?;

        success!(unsafe {
            bindings::SimConnect_SubscribeToSystemEvent(
                self.handle.as_ptr(),
                event as u32,
                event.into_c_char(),
            )
        })?;

        Ok(())
    }

    /// Request that notifications are no longer received for the specified system event.
    /// If the system event is not subscribed to, this function does nothing.
    #[tracing::instrument(
        name = "SimConnect::unsubscribe_from_system_event",
        level = "debug",
        skip(self)
    )]
    pub fn unsubscribe_from_system_event(
        &mut self,
        event: SystemEventRequest,
    ) -> Result<(), SimConnectError> {
        if self.system_event_register.is_registered(event) {
            success!(unsafe {
                bindings::SimConnect_UnsubscribeFromSystemEvent(self.handle.as_ptr(), event as u32)
            })?;

            self.system_event_register.clear();
        }

        Ok(())
    }

    /// Request that a specific client event is notified.
    #[tracing::instrument(
        name = "SimConnect::subscribe_to_client_event",
        level = "debug",
        skip(self)
    )]
    pub fn subscribe_to_client_event(
        &mut self,
        event: ClientEventRequest,
    ) -> Result<(), SimConnectError> {
        self.client_event_register.register(event)?;

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
                NOTIFICATION_GROUP_ID,
                event as u32,
                0,
            )
        })?;

        success!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(
                self.handle.as_ptr(),
                NOTIFICATION_GROUP_ID,
                bindings::SIMCONNECT_GROUP_PRIORITY_HIGHEST,
            )
        })?;

        Ok(())
    }

    /// Request that notifications are no longer received for the specified client event.
    /// If the client event is not subscribed to, this function does nothing.
    #[tracing::instrument(
        name = "SimConnect::unsubscribe_from_client_event",
        level = "debug",
        skip(self)
    )]
    pub fn unsubscribe_from_client_event(
        &mut self,
        event: ClientEventRequest,
    ) -> Result<(), SimConnectError> {
        if self.client_event_register.is_registered(event) {
            success!(unsafe {
                bindings::SimConnect_RemoveClientEvent(
                    self.handle.as_ptr(),
                    NOTIFICATION_GROUP_ID,
                    event as u32,
                )
            })?;

            self.client_event_register.unregister(event)?;
        }

        Ok(())
    }

    /// Request that notifications are no longer received for any client event.
    #[tracing::instrument(
        name = "SimConnect::unsubscribe_from_all_client_events",
        level = "debug",
        skip(self)
    )]
    pub fn unsubscribe_from_all_client_events(&mut self) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_ClearNotificationGroup(self.handle.as_ptr(), NOTIFICATION_GROUP_ID)
        })?;

        self.client_event_register.clear();

        Ok(())
    }

    /// Request that the Microsoft Flight Simulator server transmit to all SimConnect clients the specified client event.
    #[tracing::instrument(
        name = "SimConnect::transmit_client_event",
        level = "debug",
        skip(self)
    )]
    pub fn transmit_client_event(
        &mut self,
        object_id: u32,
        event_id: ClientEventRequest,
        dword: u32,
        event_flag: EventFlag,
    ) -> Result<(), SimConnectError> {
        success!(unsafe {
            bindings::SimConnect_TransmitClientEvent(self.handle.as_ptr(), object_id,  event_id as u32, dword, NOTIFICATION_GROUP_ID, event_flag as u32)
        })
    }
}
