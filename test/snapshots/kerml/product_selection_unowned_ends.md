# META
~~~ini
description=KerML Association: ProductSelection_UnownedEnds
type=file
~~~
# SOURCE
~~~kerml
package ProductSelection_UnownedEnds {
	
	class SelectionInfo;
	class ShoppingCart {
		feature selectedProducts : Product[0..*];
	}
	class Product {
		feature inCart: ShoppingCart[0..1];
	}
	
	assoc ProductSelection {
		feature info: SelectionInfo[1];
		
		end feature cart: ShoppingCart[1] crosses selectedProduct.inCart;
		end feature selectedProduct: Product[1] crosses cart.selectedProducts;
	}
	
	assoc SingleProductSelection :> ProductSelection {
		end feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
	}
	
	// Equivalent association showing implied relationships explicitly.
	assoc SingleProductSelection1 :> ProductSelection {
		end feature cart: ShoppingCart[1] redefines cart {
			public import selectedProduct::selectedProduct1;
		}
		end feature selectedProduct: Product[1] redefines selectedProduct crosses cart.selectedProduct1 {
			member feature selectedProduct1[0..1] subsets ShoppingCart::selectedProducts featured by ShoppingCart;
		}
	}
	
	class OnlineCustomer {
		feature info1: SelectionInfo;	
		feature myCart: ShoppingCart[1];	
		feature products: Product[0..*];
		
		connector ps1 : ProductSelection from myCart to products {
			:>> info = info1;
		}
		
		connector ps2 : ProductSelection from [1] myCart to [1] products {
			:>> info = info1;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "product_selection_unowned_ends.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAssoc,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,ColonGt,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwAssoc,Ident,ColonGt,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnd,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,OpenCurly,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,Ident,KwTo,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwConnector,Ident,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ProductSelection_UnownedEnds'
    (class_def 'SelectionInfo')
    (class_def 'ShoppingCart'
      (feature_def 'selectedProducts' : 'Product' multiplicity))
    (class_def 'Product'
      (feature_def 'inCart' : 'ShoppingCart' multiplicity))
    (association_def 'ProductSelection'
      (feature_def 'info' : 'SelectionInfo' multiplicity)
      (feature_def end 'cart' : 'ShoppingCart' multiplicity crosses 'selectedProduct.inCart')
      (feature_def end 'selectedProduct' : 'Product' multiplicity crosses 'cart.selectedProducts'))
    (association_def 'SingleProductSelection' :> 'ProductSelection'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity)
      (feature_def end 'selectedProduct' : 'Product' multiplicity))
    (line_comment)
    (association_def 'SingleProductSelection1' :> 'ProductSelection'
      (feature_def end 'cart' : 'ShoppingCart' multiplicity :>> 'cart'
        (import_decl public 'selectedProduct::selectedProduct1'))
      (feature_def end 'selectedProduct' : 'Product' multiplicity :>> 'selectedProduct' crosses 'cart.selectedProduct1'
        (feature_def member 'selectedProduct1' multiplicity :> 'ShoppingCart::selectedProducts' featured by 'ShoppingCart')))
    (class_def 'OnlineCustomer'
      (feature_def 'info1' : 'SelectionInfo')
      (feature_def 'myCart' : 'ShoppingCart' multiplicity)
      (feature_def 'products' : 'Product' multiplicity)
      (connector_def 'ps1' : 'ProductSelection'
        (connector_end)
        (connector_end)
        (feature_def :>> 'info' value))
      (connector_def 'ps2' : 'ProductSelection'
        (connector_end)
        (connector_end)
        (feature_def :>> 'info' value)))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
package ProductSelection_UnownedEnds {
	
	class SelectionInfo;
	class ShoppingCart {
		feature selectedProducts : Product[0..*];
	}
	class Product {
		feature inCart: ShoppingCart[0..1];
	}
	
	assoc ProductSelection {
		feature info: SelectionInfo[1];
		
		end feature cart: ShoppingCart[1] crosses selectedProduct.inCart;
		end feature selectedProduct: Product[1] crosses cart.selectedProducts;
	}
	
	assoc SingleProductSelection :> ProductSelection {
		end feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
	}
	
	// Equivalent association showing implied relationships explicitly.
	assoc SingleProductSelection1 :> ProductSelection {
		end feature cart: ShoppingCart[1] redefines cart {
			public import selectedProduct::selectedProduct1;
		}
		end feature selectedProduct: Product[1] redefines selectedProduct crosses cart.selectedProduct1 {
			member feature selectedProduct1[0..1] subsets ShoppingCart::selectedProducts featured by ShoppingCart;
		}
	}
	
	class OnlineCustomer {
		feature info1: SelectionInfo;	
		feature myCart: ShoppingCart[1];	
		feature products: Product[0..*];
		
		connector ps1 : ProductSelection from myCart to products {
			:>> info = info1;
		}
		
		connector ps2 : ProductSelection from [1] myCart to [1] products {
			:>> info = info1;
		}
	}
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ee36af175e2102ef2267226689f8e9cb031f6e99a2512b6f3637c818962fc354") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds"))) (kind "package") (name "ProductSelection_UnownedEnds") (declared-name "ProductSelection_UnownedEnds") (range (start (line 0) (character 0)) (end (line 0) (character 1311))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds::OnlineCustomer"))) (kind "classifier decl") (name "OnlineCustomer") (declared-name "OnlineCustomer") (range (start (line 32) (character 1)) (end (line 32) (character 316))) (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds::Product"))) (kind "classifier decl") (name "Product") (declared-name "Product") (range (start (line 6) (character 1)) (end (line 6) (character 57))) (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds::ProductSelection"))) (kind "kermlDecl") (name "ProductSelection") (declared-name "ProductSelection") (range (start (line 10) (character 1)) (end (line 10) (character 206))) (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo"))) (kind "classifier decl") (name "SelectionInfo") (declared-name "SelectionInfo") (range (start (line 2) (character 1)) (end (line 2) (character 21))) (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart"))) (kind "classifier decl") (name "ShoppingCart") (declared-name "ShoppingCart") (range (start (line 3) (character 1)) (end (line 3) (character 68))) (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection"))) (kind "kermlDecl") (name "SingleProductSelection") (declared-name "SingleProductSelection") (range (start (line 17) (character 1)) (end (line 17) (character 141))) (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1"))) (kind "kermlDecl") (name "SingleProductSelection1") (declared-name "SingleProductSelection1") (range (start (line 23) (character 1)) (end (line 23) (character 374))) (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
