# META
~~~ini
description=KerML Association: ProductSelection_OwnedEnds
type=file
~~~
# SOURCE
~~~kerml
package ProductSelection_OwnedEnds {
	
	class SelectionInfo;
	class ShoppingCart;
	class Product;
	
	// User-specified association definition
	assoc ProductSelection {
		feature info: SelectionInfo;
		
		end [0..1] feature cart: ShoppingCart[1];
		end [0..*] nonunique feature selectedProduct: Product[1];
	}
	
	// Equivalent association definition with named end features.
	assoc ProductSelection1 {
		feature info: SelectionInfo;
		
		end inCart[0..1] feature cart: ShoppingCart[1];
		end selectedProducts[0..*] feature selectedProduct: Product[1];
	}
	
	// Equivalent association definition with nested cross features.
	assoc ProductSelection2 {
		feature info: SelectionInfo;
		
		end feature cart: ShoppingCart[1] { 
			member feature inCart[0..1]; // owned cross feature
		}
		end feature selectedProduct: Product[1] { 
			member feature selectedProducts[0..*]; // owned cross feature
		}
	}
	
	// Equivalent association definition showing library model specialization 
	// and implied cross subsetting.
	assoc ProductSelection3 specializes Links::BinaryLink {
		feature info: SelectionInfo;
		
		end cart: ShoppingCart[1] redefines source crosses selectedProduct.inCart {
			member feature inCart: ShoppingCart[0..1] featured by Product;
			public import selectedProduct::selectedProducts;
		}
		end selectedProduct: Product[1] redefines target crosses cart.selectedProducts {
			member feature selectedProducts: Product[0..*] featured by ShoppingCart;
			public import cart::inCart;
		}
	}
	
	assoc SingleProductSelection specializes ProductSelection {
		end [0..1] feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
	}

	assoc SingleProductSelection1 specializes ProductSelection1 {
		end inCart1 [0..1] feature cart: ShoppingCart[1];
		end selectedProduct1 [0..1] feature selectedProduct: Product[1];
	}
	
	assoc SingleProductSelection2 specializes ProductSelection2 {
		end feature cart: ShoppingCart[1] {
			member feature inCart1[0..1]; // owned crossing feature
		}
		end feature selectedProduct: Product[1] {
			member feature selectedProduct1[0..1]; // owned crossing feature
		}
	}
	
	assoc SingleProductSelection3 specializes ProductSelection3 {
		end cart: ShoppingCart[1] redefines cart crosses selectedProduct.inCart1 {
			member feature inCart1[0..1] subsets inCart featured by Product;
			public import selectedProduct::selectedProduct1;
		}
		end selectedProduct: Product[1] redefines selectedProduct crosses cart.selectedProduct1 {
			member feature selectedProduct1[0..1] subsets selectedProducts featured by ShoppingCart;
			public import cart::inCart1;
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
  (document "product_selection_owned_ends.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ProductSelection_OwnedEnds {
	
	class SelectionInfo;
	class ShoppingCart;
	class Product;
	
	// User-specified association definition
	assoc ProductSelection {
		feature info: SelectionInfo;
		
		end [0..1] feature cart: ShoppingCart[1];
		end [0..*] nonunique feature selectedProduct: Product[1];
	}
	
	// Equivalent association definition with named end features.
	assoc ProductSelection1 {
		feature info: SelectionInfo;
		
		end inCart[0..1] feature cart: ShoppingCart[1];
		end selectedProducts[0..*] feature selectedProduct: Product[1];
	}
	
	// Equivalent association definition with nested cross features.
	assoc ProductSelection2 {
		feature info: SelectionInfo;
		
		end feature cart: ShoppingCart[1] { 
			member feature inCart[0..1]; // owned cross feature
		}
		end feature selectedProduct: Product[1] { 
			member feature selectedProducts[0..*]; // owned cross feature
		}
	}
	
	// Equivalent association definition showing library model specialization 
	// and implied cross subsetting.
	assoc ProductSelection3 specializes Links::BinaryLink {
		feature info: SelectionInfo;
		
		end cart: ShoppingCart[1] redefines source crosses selectedProduct.inCart {
			member feature inCart: ShoppingCart[0..1] featured by Product;
			public import selectedProduct::selectedProducts;
		}
		end selectedProduct: Product[1] redefines target crosses cart.selectedProducts {
			member feature selectedProducts: Product[0..*] featured by ShoppingCart;
			public import cart::inCart;
		}
	}
	
	assoc SingleProductSelection specializes ProductSelection {
		end [0..1] feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
	}

	assoc SingleProductSelection1 specializes ProductSelection1 {
		end inCart1 [0..1] feature cart: ShoppingCart[1];
		end selectedProduct1 [0..1] feature selectedProduct: Product[1];
	}
	
	assoc SingleProductSelection2 specializes ProductSelection2 {
		end feature cart: ShoppingCart[1] {
			member feature inCart1[0..1]; // owned crossing feature
		}
		end feature selectedProduct: Product[1] {
			member feature selectedProduct1[0..1]; // owned crossing feature
		}
	}
	
	assoc SingleProductSelection3 specializes ProductSelection3 {
		end cart: ShoppingCart[1] redefines cart crosses selectedProduct.inCart1 {
			member feature inCart1[0..1] subsets inCart featured by Product;
			public import selectedProduct::selectedProduct1;
		}
		end selectedProduct: Product[1] redefines selectedProduct crosses cart.selectedProduct1 {
			member feature selectedProduct1[0..1] subsets selectedProducts featured by ShoppingCart;
			public import cart::inCart1;
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b0585c68a3726c230abac8ce3d47231fba5b50192fef6f9aa34ad6d11b5893c1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))) (kind "package") (name "ProductSelection_OwnedEnds") (declared-name "ProductSelection_OwnedEnds") (range (start (line 0) (character 0)) (end (line 0) (character 2940))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::OnlineCustomer"))) (kind "classifier decl") (name "OnlineCustomer") (declared-name "OnlineCustomer") (range (start (line 79) (character 1)) (end (line 79) (character 316))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::Product"))) (kind "classifier decl") (name "Product") (declared-name "Product") (range (start (line 4) (character 1)) (end (line 4) (character 15))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::ProductSelection"))) (kind "kermlDecl") (name "ProductSelection") (declared-name "ProductSelection") (range (start (line 7) (character 1)) (end (line 7) (character 166))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1"))) (kind "kermlDecl") (name "ProductSelection1") (declared-name "ProductSelection1") (range (start (line 15) (character 1)) (end (line 15) (character 179))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2"))) (kind "kermlDecl") (name "ProductSelection2") (declared-name "ProductSelection2") (range (start (line 23) (character 1)) (end (line 23) (character 275))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3"))) (kind "kermlDecl") (name "ProductSelection3") (declared-name "ProductSelection3") (range (start (line 36) (character 1)) (end (line 36) (character 487))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo"))) (kind "classifier decl") (name "SelectionInfo") (declared-name "SelectionInfo") (range (start (line 2) (character 1)) (end (line 2) (character 21))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (kind "classifier decl") (name "ShoppingCart") (declared-name "ShoppingCart") (range (start (line 3) (character 1)) (end (line 3) (character 20))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection"))) (kind "kermlDecl") (name "SingleProductSelection") (declared-name "SingleProductSelection") (range (start (line 49) (character 1)) (end (line 49) (character 157))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1"))) (kind "kermlDecl") (name "SingleProductSelection1") (declared-name "SingleProductSelection1") (range (start (line 54) (character 1)) (end (line 54) (character 184))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2"))) (kind "kermlDecl") (name "SingleProductSelection2") (declared-name "SingleProductSelection2") (range (start (line 59) (character 1)) (end (line 59) (character 282))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3"))) (kind "kermlDecl") (name "SingleProductSelection3") (declared-name "SingleProductSelection3") (range (start (line 68) (character 1)) (end (line 68) (character 486))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds"))))
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
