# META
~~~ini
description=Cross-file part def resolution with stdlib
type=multi
~~~
# SOURCE
## Definitions.sysml
~~~sysml
package Definitions {
    part def Vehicle {
        attribute mass : ScalarValues::Real;
    }
}
~~~
## Usage.sysml
~~~sysml
package Usage {
    import Definitions::*;
    part v : Vehicle;
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
  (package 'Definitions'
    (part_def 'Vehicle' :> 'Parts::Part'[part_def][implied]
      (attribute_usage composite 'mass' : 'ScalarValues::Real'[datatype_def] :> 'Base::dataValues'[feature_def][implied])))
  (package 'Usage'
    (namespace_import -> 'Definitions'[package])
    (part_usage 'v' : 'Definitions::Vehicle'[part_def] :> 'Parts::parts'[part_usage][implied])))
~~~
