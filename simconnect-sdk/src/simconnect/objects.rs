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
    #[tracing::instrument(name = "SimConnect::register_object")]
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
    /// The [`crate::SimConnectObject`] macro will automatically call this method for you.
    #[tracing::instrument(name = "SimConnect::add_to_data_definition")]
    pub fn add_to_data_definition(
        &self,
        request_id: u32,
        datum_name: &str,
        units_name: &str,
        data_type: DataType,
    ) -> Result<(), SimConnectError> {
        let c_type = match data_type {
            DataType::Float64 => bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
            DataType::Bool => bindings::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_INT32,
        };

        unsafe {
            success!(bindings::SimConnect_AddToDataDefinition(
                self.handle.as_ptr(),
                request_id,
                as_c_string!(datum_name),
                as_c_string!(units_name),
                c_type,
                0.0,
                u32::MAX,
            ));
        }

        Ok(())
    }

    /// Request when the SimConnect client is to receive data values for a specific object.
    ///
    /// # Remarks
    /// The [`crate::SimConnectObject`] macro will automatically call this method for you.
    ///
    /// It is possible to change the period of a request, by re-sending the [`crate::SimConnect::request_data_on_sim_object`] call with the same `request_id` parameters, but with a new `period`.
    /// The one exception to this is the new period cannot be [`crate::Period::Once`], in this case a request with a new `request_id` should be sent.
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
                request_id,
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
