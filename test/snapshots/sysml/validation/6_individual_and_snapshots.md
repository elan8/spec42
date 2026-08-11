# META
~~~ini
description=SysML Validation (06-Individual and Snapshots): 6-Individual and Snapshots
type=file
~~~
# SOURCE
~~~sysml
package '6-Individual and Snapshots' {
	private import ScalarValues::Real;
	private import Time::DateTime;
	private import ISQ::*;
	
	package 'Part Definitions' {	
		part def 'Temporal-Spatial Reference' {
			attribute referenceTime : DateTime;
			attribute referenceCoordinateSystem;
		}
		
		/*
		 * Note that space and time coordinatization have not
		 * been fully specified yet.
		 */
		
		part def VehicleRoadContext {
			attribute t : TimeValue;
		}
		
		part def VehicleA {
			attribute mass : MassValue;
			attribute position : Real;
			attribute velocity : Real;
			attribute acceleration : Real;
			exhibit state vehicleStates {
				entry; then on;
				state on;
				then off;
				state off;
			}
		}
		
		part def Road {
			attribute angle : Real;
			attribute surfaceFriction : Real;
		}
	}
	
	package 'Individual Definitions' {
		private import 'Part Definitions'::*;
		
		/*
		 * An individual definition restricts the instances of a part def to
		 * those that are portions of the same life ("identity").
		 */
		 
		individual def 'Temporal-Spatial Reference_ID1' :> 'Temporal-Spatial Reference';
		individual def VehicleRoadContext_ID1 :> VehicleRoadContext;
		individual def VehicleA_ID1 :> VehicleA;
		individual def Road_ID1 :> Road;
	
	}
	
	package Values {	
		attribute t0 : TimeValue;
		attribute t1 : TimeValue;
		attribute tn : TimeValue;
		
		attribute m : MassValue;
		
		attribute p0 : Real;
		attribute p1 : Real;
		attribute pn : Real;
		
		attribute v0 : Real;
		attribute v1 : Real;
		attribute vn : Real;
		
		attribute a0 : Real;
		attribute a1 : Real;
		attribute an : Real;
		
		attribute theta0 : Real;
		attribute theta1 : Real;
		attribute thetan : Real;
		
		attribute sf0 : Real;
		attribute sf1 : Real;
		attribute sfn : Real;
	}
	
	package 'Individuals and Snapshots' {
		private import 'Individual Definitions'::*;
		private import Values::*;
		
		individual reference : 'Temporal-Spatial Reference_ID1' {
			/*
			 * An individual usage must be typed by an individual definition,
			 * representing the condition of that individual during some or all
			 * of its life.
			 */
		
			snapshot context_t0 : VehicleRoadContext_ID1 {
				:>> t = t0 {
					/*
					 * This is a concise notation for showing the redefinition
					 * of a attribute property.
					 */
				}
				
				snapshot vehicle_ID1_t0 : VehicleA_ID1 {
					/*
					 * A snapshot is a kind of individual usage restricted to
					 * a single instant of time.
					 */
				
					:>> mass = m;
					:>> position = p0;
					:>> velocity = v0;
					:>> acceleration = a0;
					
					exhibit vehicleStates.on {
						/*
						 * This asserts that the snapshot exhibits the referenced 
						 * state, which means that the vehicle must me in the state 
						 * at the time of the snapshot.
						 */
					}
				}
				
				snapshot road_ID1_t0 : Road_ID1 {
					:>> angle = theta0;
					:>> surfaceFriction = sf0;
				}
			}
			
			snapshot context_t1 : VehicleRoadContext_ID1 {
				:>> t = t1;
				
				snapshot vehicle_ID1_t1 : VehicleA_ID1 {
					:>> mass = m;
					:>> position = p1;
					:>> velocity = v1;
					:>> acceleration = a1;
					
					exhibit vehicleStates.on;
				}
				
				snapshot road_ID1_t1 : Road_ID1 {
					:>> angle = theta1;
					:>> surfaceFriction = sf1;
				}
			}
			
			// ...
			
			snapshot context_tn : VehicleRoadContext_ID1 {
				:>> t = tn;
				
				snapshot vehicle_ID1_tn : VehicleA_ID1 {
					:>> mass = m;
					:>> position = pn;
					:>> velocity = vn;
					:>> acceleration = an;
					
					exhibit vehicleStates.off;
				}
				
				snapshot road_ID1_tn : Road_ID1 {
					:>> angle = theta1;
					:>> surfaceFriction = sfn;
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "6_individual_and_snapshots.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 3) (end 17 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 17) (end 17 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 3) (end 21 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 20) (end 21 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 40 17) (end 40 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 47 53) (end 47 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 48 43) (end 48 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 49 33) (end 49 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 50 29) (end 50 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 2) (end 55 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 2) (end 56 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 2) (end 57 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 2) (end 59 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 83 17) (end 83 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 84 17) (end 84 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 13) (end 86 1793))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 12) (end 93 801))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 13) (end 101 498))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 13) (end 121 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 12) (end 127 364))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 13) (end 130 182))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 13) (end 139 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 147 12) (end 147 365))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 150 13) (end 150 183))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 159 13) (end 159 100))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '6-Individual and Snapshots' {
    private import ScalarValues::Real;
    private import Time::DateTime;
    private import ISQ::*;

    package 'Part Definitions' {
        part def 'Temporal-Spatial Reference' {
            attribute referenceTime : DateTime;
            attribute referenceCoordinateSystem;
        }

        /*
		 * Note that space and time coordinatization have not
		 * been fully specified yet.
		 */

        part def VehicleRoadContext {
            attribute t : TimeValue;
        }

        part def VehicleA {
            attribute mass : MassValue;
            attribute position : Real;
            attribute velocity : Real;
            attribute acceleration : Real;
            exhibit state vehicleStates {
                entry; then on;
                state on;
                then off;
                state off;
            }
        }

        part def Road {
            attribute angle : Real;
            attribute surfaceFriction : Real;
        }
    }

    package 'Individual Definitions' {
        private import 'Part Definitions'::*;

        /*
		 * An individual definition restricts the instances of a part def to
		 * those that are portions of the same life ("identity").
		 */

        individual def 'Temporal-Spatial Reference_ID1' :> 'Temporal-Spatial Reference';
        individual def VehicleRoadContext_ID1 :> VehicleRoadContext;
        individual def VehicleA_ID1 :> VehicleA;
        individual def Road_ID1 :> Road;

    }

    package Values {
        attribute t0 : TimeValue;
        attribute t1 : TimeValue;
        attribute tn : TimeValue;

        attribute m : MassValue;

        attribute p0 : Real;
        attribute p1 : Real;
        attribute pn : Real;

        attribute v0 : Real;
        attribute v1 : Real;
        attribute vn : Real;

        attribute a0 : Real;
        attribute a1 : Real;
        attribute an : Real;

        attribute theta0 : Real;
        attribute theta1 : Real;
        attribute thetan : Real;

        attribute sf0 : Real;
        attribute sf1 : Real;
        attribute sfn : Real;
    }

    package 'Individuals and Snapshots' {
        private import 'Individual Definitions'::*;
        private import Values::*;

        individual reference : 'Temporal-Spatial Reference_ID1' {
            /*
			 * An individual usage must be typed by an individual definition,
			 * representing the condition of that individual during some or all
			 * of its life.
			 */

            snapshot context_t0 : VehicleRoadContext_ID1 {
                :>> t = t0 {
                    /*
					 * This is a concise notation for showing the redefinition
					 * of a attribute property.
					 */
                }

                snapshot vehicle_ID1_t0 : VehicleA_ID1 {
                    /*
					 * A snapshot is a kind of individual usage restricted to
					 * a single instant of time.
					 */

                    :>> mass = m;
                    :>> position = p0;
                    :>> velocity = v0;
                    :>> acceleration = a0;

                    exhibit vehicleStates.on {
                        /*
						 * This asserts that the snapshot exhibits the referenced 
						 * state, which means that the vehicle must me in the state 
						 * at the time of the snapshot.
						 */
                    }
                }

                snapshot road_ID1_t0 : Road_ID1 {
                    :>> angle = theta0;
                    :>> surfaceFriction = sf0;
                }
            }

            snapshot context_t1 : VehicleRoadContext_ID1 {
                :>> t = t1;

                snapshot vehicle_ID1_t1 : VehicleA_ID1 {
                    :>> mass = m;
                    :>> position = p1;
                    :>> velocity = v1;
                    :>> acceleration = a1;

                    exhibit vehicleStates.on;
                }

                snapshot road_ID1_t1 : Road_ID1 {
                    :>> angle = theta1;
                    :>> surfaceFriction = sf1;
                }
            }

            // ...

            snapshot context_tn : VehicleRoadContext_ID1 {
                :>> t = tn;

                snapshot vehicle_ID1_tn : VehicleA_ID1 {
                    :>> mass = m;
                    :>> position = pn;
                    :>> velocity = vn;
                    :>> acceleration = an;

                    exhibit vehicleStates.off;
                }

                snapshot road_ID1_tn : Road_ID1 {
                    :>> angle = theta1;
                    :>> surfaceFriction = sfn;
                }
            }
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "877ea71df30e2d34511f6bc18bd99b788012f8fd100cbaacee42b85d67b27874") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots"))) (kind "package") (name "6-Individual and Snapshots") (declared-name "6-Individual and Snapshots") (range (start (line 0) (character 0)) (end (line 0) (character 3689))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 19))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::DateTime"))) (kind "import") (name "DateTime") (declared-name "DateTime") (range (start (line 2) (character 1)) (end (line 2) (character 31))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DateTime") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 30))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions"))) (kind "package") (name "Individual Definitions") (declared-name "Individual Definitions") (range (start (line 39) (character 1)) (end (line 39) (character 453))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 40) (character 2)) (end (line 40) (character 39))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Part Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 40) (character 17)) (end (line 40) (character 35))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Road_ID1"))) (kind "individual def") (name "Road_ID1") (declared-name "Road_ID1") (range (start (line 50) (character 2)) (end (line 50) (character 34))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Road") (range (start (line 50) (character 29)) (end (line 50) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Temporal-Spatial Reference_ID1"))) (kind "individual def") (name "Temporal-Spatial Reference_ID1") (declared-name "Temporal-Spatial Reference_ID1") (range (start (line 47) (character 2)) (end (line 47) (character 82))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Temporal-Spatial Reference") (range (start (line 47) (character 53)) (end (line 47) (character 81)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleA_ID1"))) (kind "individual def") (name "VehicleA_ID1") (declared-name "VehicleA_ID1") (range (start (line 49) (character 2)) (end (line 49) (character 42))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleA") (range (start (line 49) (character 33)) (end (line 49) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleRoadContext_ID1"))) (kind "individual def") (name "VehicleRoadContext_ID1") (declared-name "VehicleRoadContext_ID1") (range (start (line 48) (character 2)) (end (line 48) (character 62))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleRoadContext") (range (start (line 48) (character 43)) (end (line 48) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots"))) (kind "package") (name "Individuals and Snapshots") (declared-name "Individuals and Snapshots") (range (start (line 82) (character 1)) (end (line 82) (character 1912))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 83) (character 2)) (end (line 83) (character 45))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots"))) (authored (membership (kind Import) (visibility "private") (import (reference "Individual Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 83) (character 17)) (end (line 83) (character 41))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 84) (character 2)) (end (line 84) (character 27))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots"))) (authored (membership (kind Import) (visibility "private") (import (reference "Values::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 84) (character 17)) (end (line 84) (character 23))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (kind "occurrence") (name "reference") (declared-name "reference") (range (start (line 86) (character 13)) (end (line 86) (character 1793))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots"))) (authored (membership (kind Feature)) (relationships (typing (reference "Temporal-Spatial Reference_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (kind "occurrence") (name "context_t0") (declared-name "context_t0") (range (start (line 93) (character 12)) (end (line 93) (character 801))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleRoadContext_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0"))) (kind "occurrence") (name "road_ID1_t0") (declared-name "road_ID1_t0") (range (start (line 121) (character 13)) (end (line 121) (character 100))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (authored (membership (kind Feature)) (relationships (typing (reference "Road_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle"))) (kind "attribute") (name "angle") (declared-name "angle") (range (start (line 122) (character 5)) (end (line 122) (character 24))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "angle") (range (start (line 122) (character 5)) (end (line 122) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction"))) (kind "attribute") (name "surfaceFriction") (declared-name "surfaceFriction") (range (start (line 123) (character 5)) (end (line 123) (character 31))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "surfaceFriction") (range (start (line 123) (character 5)) (end (line 123) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t"))) (kind "attribute") (name "t") (declared-name "t") (range (start (line 94) (character 4)) (end (line 94) (character 136))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "t") (range (start (line 94) (character 4)) (end (line 94) (character 9)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (kind "occurrence") (name "vehicle_ID1_t0") (declared-name "vehicle_ID1_t0") (range (start (line 101) (character 13)) (end (line 101) (character 498))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration"))) (kind "attribute") (name "acceleration") (declared-name "acceleration") (range (start (line 110) (character 5)) (end (line 110) (character 27))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "acceleration") (range (start (line 110) (character 5)) (end (line 110) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 107) (character 5)) (end (line 107) (character 18))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 107) (character 5)) (end (line 107) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 108) (character 5)) (end (line 108) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "position") (range (start (line 108) (character 5)) (end (line 108) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::vehicleStates.on"))) (kind "state") (name "vehicleStates.on") (declared-name "vehicleStates.on") (range (start (line 112) (character 5)) (end (line 112) (character 227))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (range (start (line 109) (character 5)) (end (line 109) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "velocity") (range (start (line 109) (character 5)) (end (line 109) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (kind "occurrence") (name "context_t1") (declared-name "context_t1") (range (start (line 127) (character 12)) (end (line 127) (character 364))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleRoadContext_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1"))) (kind "occurrence") (name "road_ID1_t1") (declared-name "road_ID1_t1") (range (start (line 139) (character 13)) (end (line 139) (character 100))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Road_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle"))) (kind "attribute") (name "angle") (declared-name "angle") (range (start (line 140) (character 5)) (end (line 140) (character 24))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "angle") (range (start (line 140) (character 5)) (end (line 140) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction"))) (kind "attribute") (name "surfaceFriction") (declared-name "surfaceFriction") (range (start (line 141) (character 5)) (end (line 141) (character 31))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "surfaceFriction") (range (start (line 141) (character 5)) (end (line 141) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t"))) (kind "attribute") (name "t") (declared-name "t") (range (start (line 128) (character 4)) (end (line 128) (character 15))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "t") (range (start (line 128) (character 4)) (end (line 128) (character 9)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (kind "occurrence") (name "vehicle_ID1_t1") (declared-name "vehicle_ID1_t1") (range (start (line 130) (character 13)) (end (line 130) (character 182))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration"))) (kind "attribute") (name "acceleration") (declared-name "acceleration") (range (start (line 134) (character 5)) (end (line 134) (character 27))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "acceleration") (range (start (line 134) (character 5)) (end (line 134) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 131) (character 5)) (end (line 131) (character 18))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 131) (character 5)) (end (line 131) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 132) (character 5)) (end (line 132) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "position") (range (start (line 132) (character 5)) (end (line 132) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::vehicleStates.on"))) (kind "state") (name "vehicleStates.on") (declared-name "vehicleStates.on") (range (start (line 136) (character 5)) (end (line 136) (character 30))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (range (start (line 133) (character 5)) (end (line 133) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "velocity") (range (start (line 133) (character 5)) (end (line 133) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (kind "occurrence") (name "context_tn") (declared-name "context_tn") (range (start (line 147) (character 12)) (end (line 147) (character 365))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleRoadContext_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn"))) (kind "occurrence") (name "road_ID1_tn") (declared-name "road_ID1_tn") (range (start (line 159) (character 13)) (end (line 159) (character 100))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (authored (membership (kind Feature)) (relationships (typing (reference "Road_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle"))) (kind "attribute") (name "angle") (declared-name "angle") (range (start (line 160) (character 5)) (end (line 160) (character 24))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "angle") (range (start (line 160) (character 5)) (end (line 160) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction"))) (kind "attribute") (name "surfaceFriction") (declared-name "surfaceFriction") (range (start (line 161) (character 5)) (end (line 161) (character 31))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "surfaceFriction") (range (start (line 161) (character 5)) (end (line 161) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t"))) (kind "attribute") (name "t") (declared-name "t") (range (start (line 148) (character 4)) (end (line 148) (character 15))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "t") (range (start (line 148) (character 4)) (end (line 148) (character 9)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (kind "occurrence") (name "vehicle_ID1_tn") (declared-name "vehicle_ID1_tn") (range (start (line 150) (character 13)) (end (line 150) (character 183))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA_ID1") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration"))) (kind "attribute") (name "acceleration") (declared-name "acceleration") (range (start (line 154) (character 5)) (end (line 154) (character 27))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "acceleration") (range (start (line 154) (character 5)) (end (line 154) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 151) (character 5)) (end (line 151) (character 18))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 151) (character 5)) (end (line 151) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 152) (character 5)) (end (line 152) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "position") (range (start (line 152) (character 5)) (end (line 152) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::vehicleStates.off"))) (kind "state") (name "vehicleStates.off") (declared-name "vehicleStates.off") (range (start (line 156) (character 5)) (end (line 156) (character 31))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (range (start (line 153) (character 5)) (end (line 153) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "velocity") (range (start (line 153) (character 5)) (end (line 153) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions"))) (kind "package") (name "Part Definitions") (declared-name "Part Definitions") (range (start (line 5) (character 1)) (end (line 5) (character 670))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road"))) (kind "part def") (name "Road") (declared-name "Road") (range (start (line 33) (character 2)) (end (line 33) (character 85))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (kind "attribute") (name "angle") (declared-name "angle") (range (start (line 34) (character 3)) (end (line 34) (character 26))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 34) (character 21)) (end (line 34) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (kind "attribute") (name "surfaceFriction") (declared-name "surfaceFriction") (range (start (line 35) (character 3)) (end (line 35) (character 36))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 35) (character 31)) (end (line 35) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference"))) (kind "part def") (name "Temporal-Spatial Reference") (declared-name "Temporal-Spatial Reference") (range (start (line 6) (character 2)) (end (line 6) (character 124))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceCoordinateSystem"))) (kind "attribute") (name "referenceCoordinateSystem") (declared-name "referenceCoordinateSystem") (range (start (line 8) (character 3)) (end (line 8) (character 39))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (kind "attribute") (name "referenceTime") (declared-name "referenceTime") (range (start (line 7) (character 3)) (end (line 7) (character 38))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference"))) (authored (membership (kind Feature)) (relationships (typing (reference "DateTime") (range none)) (typing (reference "DateTime") (range (start (line 7) (character 29)) (end (line 7) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))) (kind "part def") (name "VehicleA") (declared-name "VehicleA") (range (start (line 20) (character 2)) (end (line 20) (character 251))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (kind "attribute") (name "acceleration") (declared-name "acceleration") (range (start (line 24) (character 3)) (end (line 24) (character 33))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 24) (character 28)) (end (line 24) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 21) (character 3)) (end (line 21) (character 30))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 21) (character 20)) (end (line 21) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 22) (character 3)) (end (line 22) (character 29))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 22) (character 24)) (end (line 22) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (kind "exhibit state") (name "vehicleStates") (declared-name "vehicleStates") (range (start (line 25) (character 3)) (end (line 25) (character 100))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))) (authored (relationships (initial-state (reference "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::on") (range none)) (initial-state (reference "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 26) (character 4)) (end (line 26) (character 10))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 29) (character 4)) (end (line 29) (character 14))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 27) (character 4)) (end (line 27) (character 13))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (range (start (line 23) (character 3)) (end (line 23) (character 29))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 23) (character 24)) (end (line 23) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext"))) (kind "part def") (name "VehicleRoadContext") (declared-name "VehicleRoadContext") (range (start (line 16) (character 2)) (end (line 16) (character 63))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t"))) (kind "attribute") (name "t") (declared-name "t") (range (start (line 17) (character 3)) (end (line 17) (character 27))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeValue") (range none)) (typing (reference "TimeValue") (range (start (line 17) (character 17)) (end (line 17) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (kind "package") (name "Values") (declared-name "Values") (range (start (line 54) (character 1)) (end (line 54) (character 510))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots"))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a0"))) (kind "attribute def") (name "a0") (declared-name "a0") (range (start (line 69) (character 2)) (end (line 69) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a1"))) (kind "attribute def") (name "a1") (declared-name "a1") (range (start (line 70) (character 2)) (end (line 70) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::an"))) (kind "attribute def") (name "an") (declared-name "an") (range (start (line 71) (character 2)) (end (line 71) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::m"))) (kind "attribute def") (name "m") (declared-name "m") (range (start (line 59) (character 2)) (end (line 59) (character 26))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p0"))) (kind "attribute def") (name "p0") (declared-name "p0") (range (start (line 61) (character 2)) (end (line 61) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p1"))) (kind "attribute def") (name "p1") (declared-name "p1") (range (start (line 62) (character 2)) (end (line 62) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::pn"))) (kind "attribute def") (name "pn") (declared-name "pn") (range (start (line 63) (character 2)) (end (line 63) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf0"))) (kind "attribute def") (name "sf0") (declared-name "sf0") (range (start (line 77) (character 2)) (end (line 77) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf1"))) (kind "attribute def") (name "sf1") (declared-name "sf1") (range (start (line 78) (character 2)) (end (line 78) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sfn"))) (kind "attribute def") (name "sfn") (declared-name "sfn") (range (start (line 79) (character 2)) (end (line 79) (character 23))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::t0"))) (kind "attribute def") (name "t0") (declared-name "t0") (range (start (line 55) (character 2)) (end (line 55) (character 27))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::t1"))) (kind "attribute def") (name "t1") (declared-name "t1") (range (start (line 56) (character 2)) (end (line 56) (character 27))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta0"))) (kind "attribute def") (name "theta0") (declared-name "theta0") (range (start (line 73) (character 2)) (end (line 73) (character 26))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta1"))) (kind "attribute def") (name "theta1") (declared-name "theta1") (range (start (line 74) (character 2)) (end (line 74) (character 26))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::thetan"))) (kind "attribute def") (name "thetan") (declared-name "thetan") (range (start (line 75) (character 2)) (end (line 75) (character 26))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::tn"))) (kind "attribute def") (name "tn") (declared-name "tn") (range (start (line 57) (character 2)) (end (line 57) (character 27))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v0"))) (kind "attribute def") (name "v0") (declared-name "v0") (range (start (line 65) (character 2)) (end (line 65) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v1"))) (kind "attribute def") (name "v1") (declared-name "v1") (range (start (line 66) (character 2)) (end (line 66) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::vn"))) (kind "attribute def") (name "vn") (declared-name "vn") (range (start (line 67) (character 2)) (end (line 67) (character 22))) (parent (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 3) (character 16)) (end (line 3) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::DateTime"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DateTime") (range (start (line 2) (character 16)) (end (line 2) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Part Definitions::*") (range (start (line 40) (character 17)) (end (line 40) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Road_ID1"))) (kind specialization) (ordinal 0)) (authored-target "Road") (range (start (line 50) (character 29)) (end (line 50) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Temporal-Spatial Reference_ID1"))) (kind specialization) (ordinal 0)) (authored-target "Temporal-Spatial Reference") (range (start (line 47) (character 53)) (end (line 47) (character 81))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleA_ID1"))) (kind specialization) (ordinal 0)) (authored-target "VehicleA") (range (start (line 49) (character 33)) (end (line 49) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleRoadContext_ID1"))) (kind specialization) (ordinal 0)) (authored-target "VehicleRoadContext") (range (start (line 48) (character 43)) (end (line 48) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Individual Definitions::*") (range (start (line 83) (character 17)) (end (line 83) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Values::*") (range (start (line 84) (character 17)) (end (line 84) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (kind featureTyping) (ordinal 0)) (authored-target "Temporal-Spatial Reference_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleRoadContext_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0"))) (kind featureTyping) (ordinal 0)) (authored-target "Road_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle"))) (kind redefinition) (ordinal 0)) (authored-target "angle") (range (start (line 122) (character 5)) (end (line 122) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction"))) (kind redefinition) (ordinal 0)) (authored-target "surfaceFriction") (range (start (line 123) (character 5)) (end (line 123) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t"))) (kind redefinition) (ordinal 0)) (authored-target "t") (range (start (line 94) (character 4)) (end (line 94) (character 9))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration"))) (kind redefinition) (ordinal 0)) (authored-target "acceleration") (range (start (line 110) (character 5)) (end (line 110) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 107) (character 5)) (end (line 107) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position"))) (kind redefinition) (ordinal 0)) (authored-target "position") (range (start (line 108) (character 5)) (end (line 108) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity"))) (kind redefinition) (ordinal 0)) (authored-target "velocity") (range (start (line 109) (character 5)) (end (line 109) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleRoadContext_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1"))) (kind featureTyping) (ordinal 0)) (authored-target "Road_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle"))) (kind redefinition) (ordinal 0)) (authored-target "angle") (range (start (line 140) (character 5)) (end (line 140) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction"))) (kind redefinition) (ordinal 0)) (authored-target "surfaceFriction") (range (start (line 141) (character 5)) (end (line 141) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t"))) (kind redefinition) (ordinal 0)) (authored-target "t") (range (start (line 128) (character 4)) (end (line 128) (character 9))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration"))) (kind redefinition) (ordinal 0)) (authored-target "acceleration") (range (start (line 134) (character 5)) (end (line 134) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 131) (character 5)) (end (line 131) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position"))) (kind redefinition) (ordinal 0)) (authored-target "position") (range (start (line 132) (character 5)) (end (line 132) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity"))) (kind redefinition) (ordinal 0)) (authored-target "velocity") (range (start (line 133) (character 5)) (end (line 133) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleRoadContext_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn"))) (kind featureTyping) (ordinal 0)) (authored-target "Road_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle"))) (kind redefinition) (ordinal 0)) (authored-target "angle") (range (start (line 160) (character 5)) (end (line 160) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction"))) (kind redefinition) (ordinal 0)) (authored-target "surfaceFriction") (range (start (line 161) (character 5)) (end (line 161) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t"))) (kind redefinition) (ordinal 0)) (authored-target "t") (range (start (line 148) (character 4)) (end (line 148) (character 9))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA_ID1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration"))) (kind redefinition) (ordinal 0)) (authored-target "acceleration") (range (start (line 154) (character 5)) (end (line 154) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 151) (character 5)) (end (line 151) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position"))) (kind redefinition) (ordinal 0)) (authored-target "position") (range (start (line 152) (character 5)) (end (line 152) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity"))) (kind redefinition) (ordinal 0)) (authored-target "velocity") (range (start (line 153) (character 5)) (end (line 153) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 34) (character 21)) (end (line 34) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 35) (character 31)) (end (line 35) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "DateTime") (range (start (line 7) (character 29)) (end (line 7) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::DateTime")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 24) (character 28)) (end (line 24) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 21) (character 20)) (end (line 21) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 22) (character 24)) (end (line 22) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::on")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (kind initialStateSource) (ordinal 1)) (authored-target "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 23) (character 24)) (end (line 23) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t"))) (kind featureTyping) (ordinal 1)) (authored-target "TimeValue") (range (start (line 17) (character 17)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a0"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a1"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::an"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p0"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::pn"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf0"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf1"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sfn"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::t0"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::t1"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta0"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta1"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::thetan"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::tn"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v0"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v1"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::vn"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::DateTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::DateTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (kind initialStateSource) (ordinal 1)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a0"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a1"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::an"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::an"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p0"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p1"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::pn"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::pn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf0"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf1"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sfn"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sfn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta0"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta1"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::thetan"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::thetan"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v0"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v0"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v1"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::vn"))) (target (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::vn"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 16) (end 3 19)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 3 16) (end 3 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 24) (end 22 28)) (probe (position 22 24))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 22 24) (end 22 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 23 24) (end 23 28)) (probe (position 23 24))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 23 24) (end 23 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 24 28) (end 24 32)) (probe (position 24 28))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 24 28) (end 24 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 34 21) (end 34 25)) (probe (position 34 21))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 34 21) (end 34 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 35 31) (end 35 35)) (probe (position 35 31))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 35 31) (end 35 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 50 29) (end 50 33)) (probe (position 50 29))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Road_ID1"))
        (kind specialization) (ordinal 0) (authored-target "Road")
        (range (start 50 29) (end 50 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 94 4) (end 94 9)) (probe (position 94 4))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t"))
        (kind redefinition) (ordinal 0) (authored-target "t")
        (range (start 94 4) (end 94 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t") (range (start 94 4) (end 94 136)))
        )
      )
    )
    (query (range (start 128 4) (end 128 9)) (probe (position 128 4))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t"))
        (kind redefinition) (ordinal 0) (authored-target "t")
        (range (start 128 4) (end 128 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t") (range (start 128 4) (end 128 15)))
        )
      )
    )
    (query (range (start 148 4) (end 148 9)) (probe (position 148 4))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t"))
        (kind redefinition) (ordinal 0) (authored-target "t")
        (range (start 148 4) (end 148 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t") (range (start 148 4) (end 148 15)))
        )
      )
    )
    (query (range (start 84 17) (end 84 23)) (probe (position 84 17))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Values::*")
        (range (start 84 17) (end 84 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 29) (end 7 37)) (probe (position 7 29))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))
        (kind featureTyping) (ordinal 1) (authored-target "DateTime")
        (range (start 7 29) (end 7 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::DateTime") (range (start 2 1) (end 2 31)))
        )
      )
    )
    (query (range (start 49 33) (end 49 41)) (probe (position 49 33))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleA_ID1"))
        (kind specialization) (ordinal 0) (authored-target "VehicleA")
        (range (start 49 33) (end 49 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 107 5) (end 107 13)) (probe (position 107 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 107 5) (end 107 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass") (range (start 107 5) (end 107 18)))
        )
      )
    )
    (query (range (start 131 5) (end 131 13)) (probe (position 131 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 131 5) (end 131 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass") (range (start 131 5) (end 131 18)))
        )
      )
    )
    (query (range (start 151 5) (end 151 13)) (probe (position 151 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 151 5) (end 151 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass") (range (start 151 5) (end 151 18)))
        )
      )
    )
    (query (range (start 17 17) (end 17 26)) (probe (position 17 17))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t"))
        (kind featureTyping) (ordinal 1) (authored-target "TimeValue")
        (range (start 17 17) (end 17 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 20) (end 21 29)) (probe (position 21 20))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 21 20) (end 21 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 122 5) (end 122 14)) (probe (position 122 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle"))
        (kind redefinition) (ordinal 0) (authored-target "angle")
        (range (start 122 5) (end 122 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle") (range (start 122 5) (end 122 24)))
        )
      )
    )
    (query (range (start 140 5) (end 140 14)) (probe (position 140 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle"))
        (kind redefinition) (ordinal 0) (authored-target "angle")
        (range (start 140 5) (end 140 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle") (range (start 140 5) (end 140 24)))
        )
      )
    )
    (query (range (start 160 5) (end 160 14)) (probe (position 160 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle"))
        (kind redefinition) (ordinal 0) (authored-target "angle")
        (range (start 160 5) (end 160 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle") (range (start 160 5) (end 160 24)))
        )
      )
    )
    (query (range (start 108 5) (end 108 17)) (probe (position 108 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position"))
        (kind redefinition) (ordinal 0) (authored-target "position")
        (range (start 108 5) (end 108 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position") (range (start 108 5) (end 108 23)))
        )
      )
    )
    (query (range (start 109 5) (end 109 17)) (probe (position 109 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity"))
        (kind redefinition) (ordinal 0) (authored-target "velocity")
        (range (start 109 5) (end 109 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity") (range (start 109 5) (end 109 23)))
        )
      )
    )
    (query (range (start 132 5) (end 132 17)) (probe (position 132 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position"))
        (kind redefinition) (ordinal 0) (authored-target "position")
        (range (start 132 5) (end 132 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position") (range (start 132 5) (end 132 23)))
        )
      )
    )
    (query (range (start 133 5) (end 133 17)) (probe (position 133 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity"))
        (kind redefinition) (ordinal 0) (authored-target "velocity")
        (range (start 133 5) (end 133 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity") (range (start 133 5) (end 133 23)))
        )
      )
    )
    (query (range (start 152 5) (end 152 17)) (probe (position 152 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position"))
        (kind redefinition) (ordinal 0) (authored-target "position")
        (range (start 152 5) (end 152 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position") (range (start 152 5) (end 152 23)))
        )
      )
    )
    (query (range (start 153 5) (end 153 17)) (probe (position 153 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity"))
        (kind redefinition) (ordinal 0) (authored-target "velocity")
        (range (start 153 5) (end 153 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity") (range (start 153 5) (end 153 23)))
        )
      )
    )
    (query (range (start 2 16) (end 2 30)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::DateTime"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
        (range (start 2 16) (end 2 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 110 5) (end 110 21)) (probe (position 110 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration"))
        (kind redefinition) (ordinal 0) (authored-target "acceleration")
        (range (start 110 5) (end 110 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration") (range (start 110 5) (end 110 27)))
        )
      )
    )
    (query (range (start 134 5) (end 134 21)) (probe (position 134 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration"))
        (kind redefinition) (ordinal 0) (authored-target "acceleration")
        (range (start 134 5) (end 134 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration") (range (start 134 5) (end 134 27)))
        )
      )
    )
    (query (range (start 154 5) (end 154 21)) (probe (position 154 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration"))
        (kind redefinition) (ordinal 0) (authored-target "acceleration")
        (range (start 154 5) (end 154 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration") (range (start 154 5) (end 154 27)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 17) (end 40 35)) (probe (position 40 17))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Part Definitions::*")
        (range (start 40 17) (end 40 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 48 43) (end 48 61)) (probe (position 48 43))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleRoadContext_ID1"))
        (kind specialization) (ordinal 0) (authored-target "VehicleRoadContext")
        (range (start 48 43) (end 48 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 123 5) (end 123 24)) (probe (position 123 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction"))
        (kind redefinition) (ordinal 0) (authored-target "surfaceFriction")
        (range (start 123 5) (end 123 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction") (range (start 123 5) (end 123 31)))
        )
      )
    )
    (query (range (start 141 5) (end 141 24)) (probe (position 141 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction"))
        (kind redefinition) (ordinal 0) (authored-target "surfaceFriction")
        (range (start 141 5) (end 141 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction") (range (start 141 5) (end 141 31)))
        )
      )
    )
    (query (range (start 161 5) (end 161 24)) (probe (position 161 5))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction"))
        (kind redefinition) (ordinal 0) (authored-target "surfaceFriction")
        (range (start 161 5) (end 161 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction") (range (start 161 5) (end 161 31)))
        )
      )
    )
    (query (range (start 83 17) (end 83 41)) (probe (position 83 17))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Individual Definitions::*")
        (range (start 83 17) (end 83 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 47 53) (end 47 81)) (probe (position 47 53))
      (reference
        (source (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Temporal-Spatial Reference_ID1"))
        (kind specialization) (ordinal 0) (authored-target "Temporal-Spatial Reference")
        (range (start 47 53) (end 47 81))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
