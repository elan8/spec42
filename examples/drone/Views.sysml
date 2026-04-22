package Views{
    import SurveillanceDrone;

    view structure: GeneralView {
        expose SurveillanceDrone::SurveillanceQuadrotorDrone;
    }

    view grid: GridView {
        expose SurveillanceDrone::SurveillanceQuadrotorDrone;
    }

    view connections: InterconnectionView {
        expose SurveillanceDrone::SurveillanceQuadrotorDrone;
    }
}