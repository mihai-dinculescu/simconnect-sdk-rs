use crate::{
    as_c_string, bindings, success, Condition, DataType, Period, SimConnect, SimConnectError,
    SimConnectObjectExt,
};

impl SimConnect {
    // Register an object with SimConnect by assigning it an unique interval `request_id` and then calling the [`crate::SimConnectObjectExt::register`] method on the struct.
    #[tracing::instrument(name = "SimConnect::register_object")]
    pub fn register_object<T: SimConnectObjectExt>(&mut self) -> Result<u32, SimConnectError> {
        let type_name: String = std::any::type_name::<T>().into();

        let id = self.new_request_id(type_name)?;

        T::register(self, id)?;

        Ok(id)
    }

    // Unregister an object with SimConnect.
    #[tracing::instrument(name = "SimConnect::unregister_object")]
    pub fn unregister_object<T: SimConnectObjectExt>(&mut self) -> Result<u32, SimConnectError> {
        let type_name: String = std::any::type_name::<T>().into();

        let request_id = self
            .registered_objects
            .get(&type_name)
            .ok_or_else(|| SimConnectError::ObjectNotRegistered(type_name.clone()))?;

        unsafe {
            success!(bindings::SimConnect_ClearDataDefinition(
                self.handle.as_ptr(),
                *request_id,
            ));
        }

        self.unregister_request_id_by_type_name(&type_name)
            .ok_or(SimConnectError::ObjectNotRegistered(type_name))
    }

    /// Add a Microsoft Flight Simulator simulation variable name to a client defined object definition.
    ///
    /// # Remarks
    /// The [`crate::SimConnectObject`] macro will automatically call this method for the struct.
    #[tracing::instrument(name = "SimConnect::add_to_data_definition")]
    pub fn add_to_data_definition(
        &self,
        request_id: u32,
        name: &str,
        unit: &str,
        data_type: DataType,
    ) -> Result<(), SimConnectError> {
        let c_type = match data_type {
            DataType::Float64 => bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            DataType::Bool => bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT32,
            DataType::String => bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING256,
        };

        unsafe {
            success!(bindings::SimConnect_AddToDataDefinition(
                self.handle.as_ptr(),
                request_id,
                as_c_string!(name),
                as_c_string!(unit),
                c_type,
                0.0,
                u32::MAX,
            ));
        }

        Ok(())
    }

    /// Request when the SimConnect client is to receive data values for a specific object.
    ///
    /// # Current limitation
    /// All objects are requested from the local user's aircraft POV.
    /// This comes with the side-effect that currently there is no way to request data for other aircraft in multiplayer.
    ///
    /// # Arguments
    /// * `request_id` - The request ID of the object.
    /// * `period` - [`crate::Period`]
    /// * `condition` - [`crate::Condition`]
    /// * `interval` - The number of period events that should elapse between transmissions of the data. `0` means the data is transmitted every Period, `1` means that the data is transmitted every other Period, etc.
    ///
    /// # Remarks
    /// The [`crate::SimConnectObject`] macro will automatically call this method for the struct.
    #[tracing::instrument(name = "SimConnect::request_data_on_sim_object")]
    pub fn request_data_on_sim_object(
        &self,
        request_id: u32,
        period: Period,
        condition: Condition,
        interval: u32,
    ) -> Result<(), SimConnectError> {
        unsafe {
            success!(bindings::SimConnect_RequestDataOnSimObject(
                self.handle.as_ptr(),
                request_id,
                request_id,
                bindings::SIMCONNECT_OBJECT_ID_USER,
                period.into(),
                condition.into(),
                0,
                interval,
                0,
            ));
        }

        Ok(())
    }
}
