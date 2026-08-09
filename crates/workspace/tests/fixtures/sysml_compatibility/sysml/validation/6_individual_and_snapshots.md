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
(model
  (namespace
    (package '6-Individual and Snapshots'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'Time::DateTime'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (package 'Part Definitions'
        (part_def 'Temporal-Spatial Reference'
          (attribute_usage composite 'referenceTime' : 'DateTime'[unresolved])
          (attribute_usage composite 'referenceCoordinateSystem'))
        (part_def 'VehicleRoadContext'
          (attribute_usage composite 't' : 'TimeValue'[unresolved]))
        (part_def 'VehicleA'
          (attribute_usage composite 'mass' : 'MassValue'[unresolved])
          (attribute_usage composite 'position' : 'Real'[unresolved])
          (attribute_usage composite 'velocity' : 'Real'[unresolved])
          (attribute_usage composite 'acceleration' : 'Real'[unresolved])
          (state_usage composite 'vehicleStates'
            (state_subaction_membership 'entry'
              (action_usage))
            (source_succession
              (reference_usage reference 'on'))
            (state_usage composite 'on')
            (source_succession
              (reference_usage reference 'off'))
            (state_usage composite 'off')))
        (part_def 'Road'
          (attribute_usage composite 'angle' : 'Real'[unresolved])
          (attribute_usage composite 'surfaceFriction' : 'Real'[unresolved])))
      (package 'Individual Definitions'
        (namespace_import private -> '6-Individual and Snapshots::Part Definitions'[package])
        (occurrence_def individual 'Temporal-Spatial Reference_ID1' :> '6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference'[part_def])
        (occurrence_def individual 'VehicleRoadContext_ID1' :> '6-Individual and Snapshots::Part Definitions::VehicleRoadContext'[part_def])
        (occurrence_def individual 'VehicleA_ID1' :> '6-Individual and Snapshots::Part Definitions::VehicleA'[part_def])
        (occurrence_def individual 'Road_ID1' :> '6-Individual and Snapshots::Part Definitions::Road'[part_def]))
      (package 'Values'
        (attribute_usage 't0' : 'TimeValue'[unresolved])
        (attribute_usage 't1' : 'TimeValue'[unresolved])
        (attribute_usage 'tn' : 'TimeValue'[unresolved])
        (attribute_usage 'm' : 'MassValue'[unresolved])
        (attribute_usage 'p0' : 'Real'[unresolved])
        (attribute_usage 'p1' : 'Real'[unresolved])
        (attribute_usage 'pn' : 'Real'[unresolved])
        (attribute_usage 'v0' : 'Real'[unresolved])
        (attribute_usage 'v1' : 'Real'[unresolved])
        (attribute_usage 'vn' : 'Real'[unresolved])
        (attribute_usage 'a0' : 'Real'[unresolved])
        (attribute_usage 'a1' : 'Real'[unresolved])
        (attribute_usage 'an' : 'Real'[unresolved])
        (attribute_usage 'theta0' : 'Real'[unresolved])
        (attribute_usage 'theta1' : 'Real'[unresolved])
        (attribute_usage 'thetan' : 'Real'[unresolved])
        (attribute_usage 'sf0' : 'Real'[unresolved])
        (attribute_usage 'sf1' : 'Real'[unresolved])
        (attribute_usage 'sfn' : 'Real'[unresolved]))
      (package 'Individuals and Snapshots'
        (namespace_import private -> '6-Individual and Snapshots::Individual Definitions'[package])
        (namespace_import private -> '6-Individual and Snapshots::Values'[package])
        (occurrence_usage individual 'reference' : '6-Individual and Snapshots::Individual Definitions::Temporal-Spatial Reference_ID1'[occurrence_def]
          (occurrence_usage composite 'context_t0' : '6-Individual and Snapshots::Individual Definitions::VehicleRoadContext_ID1'[occurrence_def]
            (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t'[attribute_usage]
              (feature_value (=)))
            (occurrence_usage composite 'vehicle_ID1_t0' : '6-Individual and Snapshots::Individual Definitions::VehicleA_ID1'[occurrence_def]
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::mass'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::position'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::velocity'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::acceleration'[attribute_usage]
                (feature_value (=)))
              (state_usage composite 'on'))
            (occurrence_usage composite 'road_ID1_t0' : '6-Individual and Snapshots::Individual Definitions::Road_ID1'[occurrence_def]
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::Road::angle'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::Road::surfaceFriction'[attribute_usage]
                (feature_value (=)))))
          (occurrence_usage composite 'context_t1' : '6-Individual and Snapshots::Individual Definitions::VehicleRoadContext_ID1'[occurrence_def]
            (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t'[attribute_usage]
              (feature_value (=)))
            (occurrence_usage composite 'vehicle_ID1_t1' : '6-Individual and Snapshots::Individual Definitions::VehicleA_ID1'[occurrence_def]
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::mass'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::position'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::velocity'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::acceleration'[attribute_usage]
                (feature_value (=)))
              (state_usage composite 'on'))
            (occurrence_usage composite 'road_ID1_t1' : '6-Individual and Snapshots::Individual Definitions::Road_ID1'[occurrence_def]
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::Road::angle'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::Road::surfaceFriction'[attribute_usage]
                (feature_value (=)))))
          (occurrence_usage composite 'context_tn' : '6-Individual and Snapshots::Individual Definitions::VehicleRoadContext_ID1'[occurrence_def]
            (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t'[attribute_usage]
              (feature_value (=)))
            (occurrence_usage composite 'vehicle_ID1_tn' : '6-Individual and Snapshots::Individual Definitions::VehicleA_ID1'[occurrence_def]
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::mass'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::position'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::velocity'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::VehicleA::acceleration'[attribute_usage]
                (feature_value (=)))
              (state_usage composite 'off'))
            (occurrence_usage composite 'road_ID1_tn' : '6-Individual and Snapshots::Individual Definitions::Road_ID1'[occurrence_def]
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::Road::angle'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> '6-Individual and Snapshots::Part Definitions::Road::surfaceFriction'[attribute_usage]
                (feature_value (=))))))))))
~~~
