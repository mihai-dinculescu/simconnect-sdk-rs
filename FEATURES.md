# Features

## General

| Feature                                 | Status  | Comment |
| --------------------------------------- | ------- | ------- |
| DispatchProc                            |         |         |
| SimConnect_Open                         | &check; |         |
| SimConnect_Close                        | &check; |         |
| SimConnect_CallDispatch                 |         |         |
| SimConnect_GetNextDispatch              | &check; |         |
| SimConnect_RequestSystemState           |         |         |
| SimConnect_MapClientEventToSimEvent     | -       | WIP     |
| SimConnect_SubscribeToSystemEvent       | &check; |         |
| SimConnect_SetSystemEventState          |         |         |
| SimConnect_UnsubscribeFromSystemEvent   | &check; |         |
| SimConnect_SetNotificationGroupPriority | -       | WIP     |

## Events And Data

| Feature                                      | Status  | Comment                             |
| -------------------------------------------- | ------- | ----------------------------------- |
| SimConnect_RequestDataOnSimObject            | &check; | Only for SIMCONNECT_OBJECT_ID_USER  |
| SimConnect_RequestDataOnSimObjectType        |         |                                     |
| SimConnect_AddClientEventToNotificationGroup | -       | WIP                                 |
| SimConnect_RemoveClientEvent                 |         |                                     |
| SimConnect_TransmitClientEvent               |         |                                     |
| SimConnect_TransmitClientEvent_EX1           |         |                                     |
| SimConnect_MapClientDataNameToID             |         |                                     |
| SimConnect_RequestClientData                 |         |                                     |
| SimConnect_CreateClientData                  |         |                                     |
| SimConnect_AddToClientDataDefinition         |         |                                     |
| SimConnect_AddToDataDefinition               | &check; | Supports `f64`, `bool` and `String` |
| SimConnect_SetClientData                     |         |                                     |
| SimConnect_SetDataOnSimObject                |         |                                     |
| SimConnect_ClearClientDataDefinition         |         |                                     |
| SimConnect_ClearDataDefinition               | &check; |                                     |
| SimConnect_MapInputEventToClientEvent        |         |                                     |
| SimConnect_RequestNotificationGroup          |         |                                     |
| SimConnect_ClearInputGroup                   |         |                                     |
| SimConnect_ClearNotificationGroup            |         |                                     |
| SimConnect_RequestReservedKey                |         |                                     |
| SimConnect_SetInputGroupPriority             |         |                                     |
| SimConnect_SetInputGroupState                |         |                                     |
| SimConnect_RemoveInputEvent                  |         |                                     |

## AI Objects

| Feature                               | Status | Comment |
| ------------------------------------- | ------ | ------- |
| SimConnect_AICreateEnrouteATCAircraft |        |         |
| SimConnect_AICreateNonATCAircraft     |        |         |
| SimConnect_AICreateParkedATCAircraft  |        |         |
| SimConnect_AICreateSimulatedObject    |        |         |
| SimConnect_AIReleaseControl           |        |         |
| SimConnect_AIRemoveObject             |        |         |
| SimConnect_AISetAircraftFlightPlan    |        |         |

## Flights

| Feature                   | Status | Comment |
| ------------------------- | ------ | ------- |
| SimConnect_FlightLoad     |        |         |
| SimConnect_FlightSave     |        |         |
| SimConnect_FlightPlanLoad |        |         |

## Debug

| Feature                         | Status | Comment |
| ------------------------------- | ------ | ------- |
| SimConnect_GetLastSentPacketID  |        |         |
| SimConnect_RequestResponseTimes |        |         |
| SimConnect_InsertString         |        |         |
| SimConnect_RetrieveString       |        |         |

## Facilities

| Feature                                | Status  | Comment |
| -------------------------------------- | ------- | ------- |
| SimConnect_AddToFacilityDefinition     |         |         |
| SimConnect_RequestFacilitesList        | &check; |         |
| SimConnect_RequestFacilitiesList_EX1   |         |         |
| SimConnect_RequestFacilityData         |         |         |
| SimConnect_SubscribeToFacilities       | &check; |         |
| SimConnect_SubscribeToFacilities_EX1   |         |         |
| SimConnect_UnsubscribeToFacilities     | &check; |         |
| SimConnect_UnsubscribeToFacilities_EX1 |         |         |

## Missions

| Feature                                | Status | Comment |
| -------------------------------------- | ------ | ------- |
| SimConnect_CompleteCustomMissionAction |        |         |
| SimConnect_ExecuteMissionAction        |        |         |
