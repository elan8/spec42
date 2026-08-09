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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,UnrestrictedName,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Semicolon,
CloseCurly,
RegularComment,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwExhibit,KwState,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
RegularComment,
KwIndividual,KwDef,UnrestrictedName,ColonGt,UnrestrictedName,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwIndividual,Ident,Colon,UnrestrictedName,OpenCurly,
RegularComment,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
RegularComment,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
KwExhibit,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
KwExhibit,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
KwExhibit,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwSnapshot,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''6-Individual and Snapshots''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Time::DateTime')
    (import_decl private 'ISQ::*')
    (package_def ''Part Definitions''
      (part_def ''Temporal-Spatial Reference''
        (attribute_usage 'referenceTime' : 'DateTime')
        (attribute_usage 'referenceCoordinateSystem'))
      (comment)
      (part_def 'VehicleRoadContext'
        (attribute_usage 't' : 'TimeValue'))
      (part_def 'VehicleA'
        (attribute_usage 'mass' : 'MassValue')
        (attribute_usage 'position' : 'Real')
        (attribute_usage 'velocity' : 'Real')
        (attribute_usage 'acceleration' : 'Real')
        (exhibit_state 'vehicleStates'
          (entry_action)
          (source_succession
            (default_ref_usage 'on'))
          (state_usage 'on')
          (source_succession
            (default_ref_usage 'off'))
          (state_usage 'off')))
      (part_def 'Road'
        (attribute_usage 'angle' : 'Real')
        (attribute_usage 'surfaceFriction' : 'Real')))
    (package_def ''Individual Definitions''
      (import_decl private ''Part Definitions'::*')
      (comment)
      (individual_def individual ''Temporal-Spatial Reference_ID1'' :> ''Temporal-Spatial Reference'')
      (individual_def individual 'VehicleRoadContext_ID1' :> 'VehicleRoadContext')
      (individual_def individual 'VehicleA_ID1' :> 'VehicleA')
      (individual_def individual 'Road_ID1' :> 'Road'))
    (package_def 'Values'
      (attribute_usage 't0' : 'TimeValue')
      (attribute_usage 't1' : 'TimeValue')
      (attribute_usage 'tn' : 'TimeValue')
      (attribute_usage 'm' : 'MassValue')
      (attribute_usage 'p0' : 'Real')
      (attribute_usage 'p1' : 'Real')
      (attribute_usage 'pn' : 'Real')
      (attribute_usage 'v0' : 'Real')
      (attribute_usage 'v1' : 'Real')
      (attribute_usage 'vn' : 'Real')
      (attribute_usage 'a0' : 'Real')
      (attribute_usage 'a1' : 'Real')
      (attribute_usage 'an' : 'Real')
      (attribute_usage 'theta0' : 'Real')
      (attribute_usage 'theta1' : 'Real')
      (attribute_usage 'thetan' : 'Real')
      (attribute_usage 'sf0' : 'Real')
      (attribute_usage 'sf1' : 'Real')
      (attribute_usage 'sfn' : 'Real'))
    (package_def ''Individuals and Snapshots''
      (import_decl private ''Individual Definitions'::*')
      (import_decl private 'Values::*')
      (individual_usage individual 'reference' : ''Temporal-Spatial Reference_ID1''
        (comment)
        (portion_usage snapshot 'context_t0' : 'VehicleRoadContext_ID1'
          (default_ref_usage :>> 't' value
            (comment))
          (portion_usage snapshot 'vehicle_ID1_t0' : 'VehicleA_ID1'
            (comment)
            (default_ref_usage :>> 'mass' value)
            (default_ref_usage :>> 'position' value)
            (default_ref_usage :>> 'velocity' value)
            (default_ref_usage :>> 'acceleration' value)
            (exhibit_state 'on'
              (comment)))
          (portion_usage snapshot 'road_ID1_t0' : 'Road_ID1'
            (default_ref_usage :>> 'angle' value)
            (default_ref_usage :>> 'surfaceFriction' value)))
        (portion_usage snapshot 'context_t1' : 'VehicleRoadContext_ID1'
          (default_ref_usage :>> 't' value)
          (portion_usage snapshot 'vehicle_ID1_t1' : 'VehicleA_ID1'
            (default_ref_usage :>> 'mass' value)
            (default_ref_usage :>> 'position' value)
            (default_ref_usage :>> 'velocity' value)
            (default_ref_usage :>> 'acceleration' value)
            (exhibit_state 'on'))
          (portion_usage snapshot 'road_ID1_t1' : 'Road_ID1'
            (default_ref_usage :>> 'angle' value)
            (default_ref_usage :>> 'surfaceFriction' value)))
        (line_comment)
        (portion_usage snapshot 'context_tn' : 'VehicleRoadContext_ID1'
          (default_ref_usage :>> 't' value)
          (portion_usage snapshot 'vehicle_ID1_tn' : 'VehicleA_ID1'
            (default_ref_usage :>> 'mass' value)
            (default_ref_usage :>> 'position' value)
            (default_ref_usage :>> 'velocity' value)
            (default_ref_usage :>> 'acceleration' value)
            (exhibit_state 'off'))
          (portion_usage snapshot 'road_ID1_tn' : 'Road_ID1'
            (default_ref_usage :>> 'angle' value)
            (default_ref_usage :>> 'surfaceFriction' value)))))))
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
# EXPECTED
~~~
semantic.duplicate_name 'on'
semantic.duplicate_name 'off'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'on'
semantic.duplicate_name 'off'
semantic.unresolved_name 'DateTime'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "6-Individual and Snapshots"))) (name "6-Individual and Snapshots") (declared-name "6-Individual and Snapshots")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::DateTime"))) (name "DateTime") (declared-name "DateTime"))
        (element (kind "package") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions"))) (name "Individual Definitions") (declared-name "Individual Definitions")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::*"))) (name "*") (declared-name "*"))
            (element (kind "individual def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Road_ID1"))) (name "Road_ID1") (declared-name "Road_ID1"))
            (element (kind "individual def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Temporal-Spatial Reference_ID1"))) (name "Temporal-Spatial Reference_ID1") (declared-name "Temporal-Spatial Reference_ID1"))
            (element (kind "individual def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleA_ID1"))) (name "VehicleA_ID1") (declared-name "VehicleA_ID1"))
            (element (kind "individual def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleRoadContext_ID1"))) (name "VehicleRoadContext_ID1") (declared-name "VehicleRoadContext_ID1"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots"))) (name "Individuals and Snapshots") (declared-name "Individuals and Snapshots")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::*#import"))) (name "*") (declared-name "*"))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (name "reference") (declared-name "reference") (declared (properties (individual true) (composite true) (reference false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (name "context_t0") (declared-name "context_t0") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0"))) (name "road_ID1_t0") (declared-name "road_ID1_t0") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::angle"))) (name "angle") (declared-name "angle") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0::surfaceFriction"))) (name "surfaceFriction") (declared-name "surfaceFriction") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::t"))) (name "t") (declared-name "t") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (name "vehicle_ID1_t0") (declared-name "vehicle_ID1_t0") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::acceleration"))) (name "acceleration") (declared-name "acceleration") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::mass"))) (name "mass") (declared-name "mass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::position"))) (name "position") (declared-name "position") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::vehicleStates.on"))) (name "vehicleStates.on") (declared-name "vehicleStates.on") (declared (properties (composite true) (reference false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0::velocity"))) (name "velocity") (declared-name "velocity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (name "context_t1") (declared-name "context_t1") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1"))) (name "road_ID1_t1") (declared-name "road_ID1_t1") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::angle"))) (name "angle") (declared-name "angle") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1::surfaceFriction"))) (name "surfaceFriction") (declared-name "surfaceFriction") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::t"))) (name "t") (declared-name "t") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (name "vehicle_ID1_t1") (declared-name "vehicle_ID1_t1") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::acceleration"))) (name "acceleration") (declared-name "acceleration") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::mass"))) (name "mass") (declared-name "mass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::position"))) (name "position") (declared-name "position") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::vehicleStates.on"))) (name "vehicleStates.on") (declared-name "vehicleStates.on") (declared (properties (composite true) (reference false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1::velocity"))) (name "velocity") (declared-name "velocity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (name "context_tn") (declared-name "context_tn") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn"))) (name "road_ID1_tn") (declared-name "road_ID1_tn") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::angle"))) (name "angle") (declared-name "angle") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn::surfaceFriction"))) (name "surfaceFriction") (declared-name "surfaceFriction") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::t"))) (name "t") (declared-name "t") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (name "vehicle_ID1_tn") (declared-name "vehicle_ID1_tn") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot")))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::acceleration"))) (name "acceleration") (declared-name "acceleration") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::mass"))) (name "mass") (declared-name "mass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::position"))) (name "position") (declared-name "position") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::vehicleStates.off"))) (name "vehicleStates.off") (declared-name "vehicleStates.off") (declared (properties (composite true) (reference false))))
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn::velocity"))) (name "velocity") (declared-name "velocity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions"))) (name "Part Definitions") (declared-name "Part Definitions")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road"))) (name "Road") (declared-name "Road") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (name "angle") (declared-name "angle") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (name "surfaceFriction") (declared-name "surfaceFriction") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference"))) (name "Temporal-Spatial Reference") (declared-name "Temporal-Spatial Reference") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceCoordinateSystem"))) (name "referenceCoordinateSystem") (declared-name "referenceCoordinateSystem") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (name "referenceTime") (declared-name "referenceTime") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))) (name "VehicleA") (declared-name "VehicleA") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (name "acceleration") (declared-name "acceleration") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (name "position") (declared-name "position") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA")))))
                (element (kind "exhibit state") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (name "vehicleStates") (declared-name "vehicleStates") (effective (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (name "velocity") (declared-name "velocity") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext"))) (name "VehicleRoadContext") (declared-name "VehicleRoadContext") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t"))) (name "t") (declared-name "t") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "package") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values"))) (name "Values") (declared-name "Values")
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a0"))) (name "a0") (declared-name "a0") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::a1"))) (name "a1") (declared-name "a1") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::an"))) (name "an") (declared-name "an") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::m"))) (name "m") (declared-name "m") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p0"))) (name "p0") (declared-name "p0") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::p1"))) (name "p1") (declared-name "p1") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::pn"))) (name "pn") (declared-name "pn") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf0"))) (name "sf0") (declared-name "sf0") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sf1"))) (name "sf1") (declared-name "sf1") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::sfn"))) (name "sfn") (declared-name "sfn") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::t0"))) (name "t0") (declared-name "t0") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::t1"))) (name "t1") (declared-name "t1") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta0"))) (name "theta0") (declared-name "theta0") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::theta1"))) (name "theta1") (declared-name "theta1") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::thetan"))) (name "thetan") (declared-name "thetan") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::tn"))) (name "tn") (declared-name "tn") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v0"))) (name "v0") (declared-name "v0") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::v1"))) (name "v1") (declared-name "v1") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "6-Individual and Snapshots::Values::vn"))) (name "vn") (declared-name "vn") (declared (properties (ordered false) (unique true))))
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (to (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::off"))))
    (initialState (status resolved) (from (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates"))) (to (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::vehicleStates::on"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Road_ID1"))) (to (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Road"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::Temporal-Spatial Reference_ID1"))) (to (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleA_ID1"))) (to (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "6-Individual and Snapshots::Individual Definitions::VehicleRoadContext_ID1"))) (to (node (document "d0") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
