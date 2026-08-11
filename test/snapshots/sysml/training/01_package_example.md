# META
~~~ini
description=SysML Training 01 (Packages): Package Example
type=file
~~~
# SOURCE
~~~sysml
package 'Package Example' {
	public import ISQ::TorqueValue;
	private import ScalarValues::*;
	 
	private part def Automobile;
	
	public alias Car for Automobile;	                         
	alias Torque for ISQ::TorqueValue;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "01_package_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 28))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Package Example' {
    public import ISQ::TorqueValue;
    private import ScalarValues::*;

    private part def Automobile;

    public alias Car for Automobile;
    alias Torque for ISQ::TorqueValue;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6a98d27a17982c900449adced1dd81ab7290b27d9f2d02528afa9580e9c46d6a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Package Example"))) (kind "package") (name "Package Example") (declared-name "Package Example") (range (start (line 0) (character 0)) (end (line 0) (character 226))))
    (element (id (node (document "d0") (qualified-name "Package Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "Package Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Package Example::Automobile"))) (kind "part def") (name "Automobile") (declared-name "Automobile") (range (start (line 4) (character 1)) (end (line 4) (character 29))) (parent (node (document "d0") (qualified-name "Package Example"))))
    (element (id (node (document "d0") (qualified-name "Package Example::Car"))) (kind "alias") (name "Car") (declared-name "Car") (range (start (line 6) (character 1)) (end (line 6) (character 33))) (parent (node (document "d0") (qualified-name "Package Example"))))
    (element (id (node (document "d0") (qualified-name "Package Example::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (range (start (line 7) (character 1)) (end (line 7) (character 35))) (parent (node (document "d0") (qualified-name "Package Example"))))
    (element (id (node (document "d0") (qualified-name "Package Example::TorqueValue"))) (kind "import") (name "TorqueValue") (declared-name "TorqueValue") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Package Example"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQ::TorqueValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 31))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Package Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Package Example::TorqueValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::TorqueValue") (range (start (line 1) (character 15)) (end (line 1) (character 31))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 28)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Package Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 2 16) (end 2 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 15) (end 1 31)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "Package Example::TorqueValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::TorqueValue")
        (range (start 1 15) (end 1 31))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
