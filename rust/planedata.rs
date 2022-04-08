mod planeType;
mod planeData{
    struct coordinate{
        x: i32,
        y: i32,
    }
    struct PlaneData {
        flightId: String,
        modelName: String,
        currentAltitude: i64,
        currentSpeed: i64,
        arrivalDeparture: bool,
        destination: String,
        holdLandTake: i64,
        heading: i64,
        planeType: *mut planeType,
        hasAltClearance: bool,
        hasDestClearance: bool,
        flight_path: Vec<coordinate>,
    }

    impl planeData{
        pub fn new(
            flightId: String, //random
            modelName: String,
            currentAltitude: i64, //random
            currentSpeed: i64,    //cruise if arival/ 0 if departure
            arrivalDeparture: bool, //random
            destination: String, //random
            holdLandTake: i64,
            heading: i64,  //derived from destination
            planeType: *mut planeType, //passed
            hasAltClearance: bool,
            hasDestClearance: bool,
            flight_path: Vec<coordinate>,
        ) -> planeData {
            planeData {
                flightId,
                modelName,
                currentAltitude,
                currentSpeed,
                arrivalDeparture,
                destination,
                holdLandTake,
                heading,
                planeType,
                hasAltClearance,
                hasDestClearance,
                flight_path,
            }
        }
    }

    pub fn createFlight() -> planeData{
        //create plane type obect from file
        planetype* p = planeType::new(
            "Boeing 747".to_string(), //modelName
            1000, //climbRate
            1000, //turnRate
            1000, //oppCeiling
            1000, //maxSpeed
            1000, //takeoffSpeed
            1000, //stallSpeed
            1000, //cruiseSpeed
        )

        //randomize flight data

        //return planeData
        planeData::new(
            "".to_string(),
            "".to_string(),
            0,
            0,
            false,
            "".to_string(),
            0,
            0,
            0,
            false,
            false,
            vec![],
        )
    }

    pub fn getFlightId(planeData: &PlaneData) -> String {
        planeData.flightId.clone()
    }

    pub fn 
}