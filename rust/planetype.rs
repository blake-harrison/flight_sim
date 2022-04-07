mod planetype {
    struct PlaneType {
        modelName: String,
        climbRate: i64,
        turnRate: i64,
        oppCeiling: i64,
        maxSpeed: i64,
        takeoffSpeed: i64,
        stallSpeed: i64,
        cruiseSpeed: i64,
    }

    impl PlaneType {
        fn new(
            modelName: String,
            climbRate: i64,
            turnRate: i64,
            oppCeiling: i64,
            maxSpeed: i64,
            takeoffSpeed: i64,
            stallSpeed: i64,
            cruiseSpeed: i64,
        ) -> PlaneType {
            PlaneType {
                modelName,
                climbRate,
                turnRate,
                oppCeiling,
                maxSpeed,
                takeoffSpeed,
                stallSpeed,
                cruiseSpeed,
            }
        }
    }

    pub fn getModelName(planeType: &PlaneType) -> String {
        planeType.modelName.clone()
    }

    pub fn getClimbRate(planeType: &PlaneType) -> i64 {
        planeType.climbRate
    }

    pub fn getTurnRate(planeType: &PlaneType) -> i64 {
        planeType.turnRate
    }

    pub fn getOppCeiling(planeType: &PlaneType) -> i64 {
        planeType.oppCeiling
    }

    pub fn getMaxSpeed(planeType: &PlaneType) -> i64 {
        planeType.maxSpeed
    }

    pub fn getTakeoffSpeed(planeType: &PlaneType) -> i64 {
        planeType.takeoffSpeed
    }

    pub fn getStallSpeed(planeType: &PlaneType) -> i64 {
        planeType.stallSpeed
    }

    pub fn getCruiseSpeed(planeType: &PlaneType) -> i64 {
        planeType.cruiseSpeed
    }
}