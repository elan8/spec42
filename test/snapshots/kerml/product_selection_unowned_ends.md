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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5b89bd65886a6c893e85c52221f0850fc4e785839d43b201091206f05d1d81d4") (contract-version "canonical-resolution-v1"))
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
# NAVIGATION
~~~sexpr
(navigation
)
~~~
