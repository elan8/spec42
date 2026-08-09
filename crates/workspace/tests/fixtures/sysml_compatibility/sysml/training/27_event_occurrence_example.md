# META
~~~ini
description=SysML Training 27 (Occurrences): Event Occurrence Example
type=file
~~~
# SOURCE
~~~sysml
package 'Event Occurrence Example' {	
	part def Driver;
	part def CruiseController;
	part def Speedometer;
	part def Engine;
	part def Vehicle;
	
	part driver : Driver {
		event occurrence setSpeedSent;
	}
	
	part vehicle : Vehicle {
	
		part cruiseController : CruiseController {
			event occurrence setSpeedReceived;		
			then event occurrence sensedSpeedReceived;		
			then event occurrence fuelCommandSent;
		}
		
		part speedometer : Speedometer {
			event occurrence sensedSpeedSent;
		}
		
		part engine : Engine {
			event occurrence fuelCommandReceived;
		}
	
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Event Occurrence Example''
    (part_def 'Driver')
    (part_def 'CruiseController')
    (part_def 'Speedometer')
    (part_def 'Engine')
    (part_def 'Vehicle')
    (part_usage 'driver' : 'Driver'
      (event_occurrence 'setSpeedSent'))
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'cruiseController' : 'CruiseController'
        (event_occurrence 'setSpeedReceived')
        (source_succession
          (event_occurrence 'sensedSpeedReceived'))
        (source_succession
          (event_occurrence 'fuelCommandSent')))
      (part_usage 'speedometer' : 'Speedometer'
        (event_occurrence 'sensedSpeedSent'))
      (part_usage 'engine' : 'Engine'
        (event_occurrence 'fuelCommandReceived')))))
~~~
# FORMAT
~~~sysml
package 'Event Occurrence Example' {
    part def Driver;
    part def CruiseController;
    part def Speedometer;
    part def Engine;
    part def Vehicle;

    part driver : Driver {
        event occurrence setSpeedSent;
    }

    part vehicle : Vehicle {
        part cruiseController : CruiseController {
            event occurrence setSpeedReceived;
            then event occurrence sensedSpeedReceived;
            then event occurrence fuelCommandSent;
        }

        part speedometer : Speedometer {
            event occurrence sensedSpeedSent;
        }

        part engine : Engine {
            event occurrence fuelCommandReceived;
        }
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package 'Event Occurrence Example'
      (part_def 'Driver')
      (part_def 'CruiseController')
      (part_def 'Speedometer')
      (part_def 'Engine')
      (part_def 'Vehicle')
      (part_usage 'driver' : 'Event Occurrence Example::Driver'[part_def]
        (event_occurrence_usage 'setSpeedSent'))
      (part_usage 'vehicle' : 'Event Occurrence Example::Vehicle'[part_def]
        (part_usage composite 'cruiseController' : 'Event Occurrence Example::CruiseController'[part_def]
          (event_occurrence_usage 'setSpeedReceived')
          (source_succession
            (event_occurrence_usage 'sensedSpeedReceived'))
          (source_succession
            (event_occurrence_usage 'fuelCommandSent')))
        (part_usage composite 'speedometer' : 'Event Occurrence Example::Speedometer'[part_def]
          (event_occurrence_usage 'sensedSpeedSent'))
        (part_usage composite 'engine' : 'Event Occurrence Example::Engine'[part_def]
          (event_occurrence_usage 'fuelCommandReceived'))))))
~~~
