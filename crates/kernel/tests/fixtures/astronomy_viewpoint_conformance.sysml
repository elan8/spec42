package AstronomyViews {
  part def CelestialBody;
  part def Orbit {
    ref centralBody : CelestialBody;
    ref orbitingBody : CelestialBody;
  }
  part def SolarSystem {
    part sun : CelestialBody;
    part earth : CelestialBody;
    part earthOrbit : Orbit;
  }

  viewpoint def OrbitalStructureViewpoint;
  view def OrbitalStructureView;

  view solarSystemStructure : OrbitalStructureView {
    expose AstronomyViews::SolarSystem::**;
  }
  satisfy solarSystemStructure by OrbitalStructureViewpoint;
}
