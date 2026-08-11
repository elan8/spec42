# META
~~~ini
description=SysML Training 35 (Use Cases): Use Case Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Use Case Usage Example' {
	
	private import 'Use Case Definition Example'::*;
	
	part def 'Fuel Station';
	
	use case 'provide transportation' : 'Provide Transportation' {
	    subject vehicle;
	    	
		first start;
		
		then include use case 'enter vehicle' : 'Enter Vehicle' {
		    subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then use case 'drive vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor environment = 'provide transportation'::environment;
			
			include 'add fuel'[0..*] { 
                subject vehicle;
				actor fueler = driver;
			}
		}
		
		then include use case 'exit vehicle' : 'Exit Vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then done;		
	}
	
	use case 'add fuel' {
		subject vehicle : Vehicle;
		actor fueler : Person;
		actor 'fuel station' : 'Fuel Station';
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "35_use_case_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 1) (end 6 820))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 5) (end 7 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 12) (end 18 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 16) (end 23 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 2) (end 38 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 2) (end 39 24))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Use Case Usage Example' {
	
	private import 'Use Case Definition Example'::*;
	
	part def 'Fuel Station';
	
	use case 'provide transportation' : 'Provide Transportation' {
	    subject vehicle;
	    	
		first start;
		
		then include use case 'enter vehicle' : 'Enter Vehicle' {
		    subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then use case 'drive vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor environment = 'provide transportation'::environment;
			
			include 'add fuel'[0..*] { 
                subject vehicle;
				actor fueler = driver;
			}
		}
		
		then include use case 'exit vehicle' : 'Exit Vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then done;		
	}
	
	use case 'add fuel' {
		subject vehicle : Vehicle;
		actor fueler : Person;
		actor 'fuel station' : 'Fuel Station';
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2a17bbaa7e6013b49850b2740b8a51e2143815cfbeb680a8148c3ede4ffeaae5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example"))) (kind "package") (name "Use Case Usage Example") (declared-name "Use Case Usage Example") (range (start (line 0) (character 0)) (end (line 0) (character 1062))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 49))) (parent (node (document "d0") (qualified-name "Use Case Usage Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Use Case Definition Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::Fuel Station"))) (kind "part def") (name "Fuel Station") (declared-name "Fuel Station") (range (start (line 4) (character 1)) (end (line 4) (character 25))) (parent (node (document "d0") (qualified-name "Use Case Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::add fuel"))) (kind "use case") (name "add fuel") (declared-name "add fuel") (range (start (line 37) (character 1)) (end (line 37) (character 120))) (parent (node (document "d0") (qualified-name "Use Case Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (kind "actor") (name "fuel station") (declared-name "fuel station") (range (start (line 40) (character 2)) (end (line 40) (character 40))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::add fuel"))) (authored (membership (kind Actor)) (relationships (typing (reference "Fuel Station") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fueler"))) (kind "actor") (name "fueler") (declared-name "fueler") (range (start (line 39) (character 2)) (end (line 39) (character 24))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::add fuel"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 38) (character 2)) (end (line 38) (character 28))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::add fuel"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (kind "use case") (name "provide transportation") (declared-name "provide transportation") (range (start (line 6) (character 1)) (end (line 6) (character 820))) (parent (node (document "d0") (qualified-name "Use Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Provide Transportation") (range none)) (flow (reference "Use Case Usage Example::provide transportation::start") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::_verdict"))) (kind "verdict") (name "done") (declared-name "done") (range (start (line 34) (character 2)) (end (line 34) (character 12))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (kind "use case") (name "drive vehicle") (declared-name "drive vehicle") (range (start (line 17) (character 2)) (end (line 17) (character 280))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (authored (membership (kind Feature)) (relationships (flow (reference "Use Case Usage Example::provide transportation::_verdict") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel"))) (kind "include use case") (name "add fuel") (declared-name "add fuel") (range (start (line 22) (character 3)) (end (line 22) (character 95))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (authored (relationships (typing (reference "add fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 23) (character 16)) (end (line 23) (character 32))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 18) (character 12)) (end (line 18) (character 28))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start"))) (kind "succession") (name "start") (declared-name "start") (range (start (line 9) (character 2)) (end (line 9) (character 14))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (authored (relationships (flow (reference "Use Case Usage Example::provide transportation::drive vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 7) (character 5)) (end (line 7) (character 21))) (parent (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (authored (relationships (typing (reference "") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Use Case Definition Example::*") (range (start (line 2) (character 16)) (end (line 2) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel Station") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Usage Example::Fuel Station")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fueler"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Transportation") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (kind flowSource) (ordinal 0)) (authored-target "Use Case Usage Example::provide transportation::start") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "Use Case Usage Example::provide transportation::_verdict") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::_verdict")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel"))) (kind featureTyping) (ordinal 0)) (authored-target "add fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start"))) (kind flowSource) (ordinal 0)) (authored-target "Use Case Usage Example::provide transportation::drive vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (target (node (document "d0") (qualified-name "Use Case Usage Example::Fuel Station"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (target (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (target (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::_verdict"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel"))) (target (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start"))) (target (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start"))) (kind flowSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
