# META
~~~ini
description=SysML Example (Geometry): CarWithEnvelopingShape
type=file
~~~
# SOURCE
~~~sysml
package CarWithEnvelopingShape {
	private import ShapeItems::Box;
	private import SI::mm;

	part def Car {
		doc
		/*
		 * Example car with simple enveloping shape that is a solid box
		 */
	
		item boundingBox : Box [1] :> boundingShapes {
			:>> length = 4800 [mm];
			:>> width  = 1840 [mm];
			:>> height = 1350 [mm];
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'CarWithEnvelopingShape'
    (import_decl private 'ShapeItems::Box')
    (import_decl private 'SI::mm')
    (part_def 'Car'
      (documentation)
      (item_usage 'boundingBox' : 'Box' :> 'boundingShapes' multiplicity
        (default_ref_usage :>> 'length' value)
        (default_ref_usage :>> 'width' value)
        (default_ref_usage :>> 'height' value)))))
~~~
# FORMAT
~~~sysml
package CarWithEnvelopingShape {
    private import ShapeItems::Box;
    private import SI::mm;

    part def Car {
        doc /*
		 * Example car with simple enveloping shape that is a solid box
		 */

        item boundingBox : Box :> boundingShapes [1] {
            :>> length = 4800 [mm];
            :>> width = 1840 [mm];
            :>> height = 1350 [mm];
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Box'
semantic.unresolved_name 'boundingShapes'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Box'
semantic.unresolved_name 'boundingShapes'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
~~~
# SMG
~~~
(model
  (namespace
    (package 'CarWithEnvelopingShape'
      (membership_import private -> 'ShapeItems::Box'[unresolved])
      (membership_import private -> 'SI::mm'[unresolved])
      (part_def 'Car'
        (documentation)
        (item_usage composite 'boundingBox' : 'Box'[unresolved] :> 'boundingShapes'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'length'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'width'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'height'[unresolved]
            (feature_value (=))))))))
~~~
