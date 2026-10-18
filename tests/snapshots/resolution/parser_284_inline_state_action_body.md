# META
~~~ini
description=AST 284 retains anonymous inline state action bodies instead of reporting the old parser limitation
libraries=none
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package P { state def S { entry { state inner; } } }
~~~
