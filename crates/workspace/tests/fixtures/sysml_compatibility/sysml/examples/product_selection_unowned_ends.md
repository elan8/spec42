# META
~~~ini
description=SysML Example (Association): ProductSelection_UnownedEnds
type=file
~~~
# SOURCE
~~~sysml
package ProductSelection_UnownedEnds_SysML {
	
	item def SelectionInfo;
	item def ShoppingCart {
		item selectedProducts : Product[0..*];
	}
	item def Product {
		item inCart: ShoppingCart[0..1];
	}
	
	connection def ProductSelection {
		item info: SelectionInfo[1];
		
		end item cart: ShoppingCart[1] crosses selectedProduct.inCart;
		end item selectedProduct: Product[1] crosses cart.selectedProducts;
	}
	
	connection def SingleProductSelection :> ProductSelection {
		end item cart: ShoppingCart[1];
		end [0..1] item selectedProduct: Product[1];
	}
	
	item def OnlineCustomer {
		item info1: SelectionInfo;	
		item myCart: ShoppingCart[1];	
		item products: Product[0..*];
		
		connection ps1 : ProductSelection connect myCart to products {
			:>> info = info1;
		}
		
		connection ps2 : ProductSelection connect [1] myCart to [1] products {
			:>> info = info1;
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwEnd,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwConnection,Ident,Colon,Ident,KwConnect,Ident,KwTo,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwConnection,Ident,Colon,Ident,KwConnect,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ProductSelection_UnownedEnds_SysML'
    (item_def 'SelectionInfo')
    (item_def 'ShoppingCart'
      (item_usage 'selectedProducts' : 'Product' multiplicity))
    (item_def 'Product'
      (item_usage 'inCart' : 'ShoppingCart' multiplicity))
    (connection_def 'ProductSelection'
      (item_usage 'info' : 'SelectionInfo' multiplicity)
      (malformed)
      (malformed))
    (connection_def 'SingleProductSelection' :> 'ProductSelection'
      (interface_end end 'cart' : 'ShoppingCart')
      (interface_end end 'selectedProduct' : 'Product' multiplicity))
    (item_def 'OnlineCustomer'
      (item_usage 'info1' : 'SelectionInfo')
      (item_usage 'myCart' : 'ShoppingCart' multiplicity)
      (item_usage 'products' : 'Product' multiplicity)
      (connection_usage 'ProductSelection' 'ps1'
        (connector_end)
        (connector_end)
        (default_ref_usage :>> 'info' value))
      (connection_usage 'ProductSelection' 'ps2'
        (connector_end)
        (connector_end)
        (default_ref_usage :>> 'info' value)))))
~~~
# FORMAT
~~~sysml
package ProductSelection_UnownedEnds_SysML {
	
	item def SelectionInfo;
	item def ShoppingCart {
		item selectedProducts : Product[0..*];
	}
	item def Product {
		item inCart: ShoppingCart[0..1];
	}
	
	connection def ProductSelection {
		item info: SelectionInfo[1];
		
		end item cart: ShoppingCart[1] crosses selectedProduct.inCart;
		end item selectedProduct: Product[1] crosses cart.selectedProducts;
	}
	
	connection def SingleProductSelection :> ProductSelection {
		end item cart: ShoppingCart[1];
		end [0..1] item selectedProduct: Product[1];
	}
	
	item def OnlineCustomer {
		item info1: SelectionInfo;	
		item myCart: ShoppingCart[1];	
		item products: Product[0..*];
		
		connection ps1 : ProductSelection connect myCart to products {
			:>> info = info1;
		}
		
		connection ps2 : ProductSelection connect [1] myCart to [1] products {
			:>> info = info1;
		}
	}
	
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML"))) (name "ProductSelection_UnownedEnds_SysML") (declared-name "ProductSelection_UnownedEnds_SysML")
      (contains
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer"))) (name "OnlineCustomer") (declared-name "OnlineCustomer"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (name "Product") (declared-name "Product"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (name "ProductSelection") (declared-name "ProductSelection")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (name "cart") (declared-name "cart") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (name "selectedProduct") (declared-name "selectedProduct") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo"))) (name "SelectionInfo") (declared-name "SelectionInfo"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (name "ShoppingCart") (declared-name "ShoppingCart"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (name "SingleProductSelection") (declared-name "SingleProductSelection")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (name "cart") (declared-name "cart") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (name "selectedProduct") (declared-name "selectedProduct") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection")))))
          )
        )
      )
    )
  )
  (relationships
    (connection (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (to (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (to (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (to (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (to (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (to (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (to (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (to (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (status missing-prerequisite) (target "Connections::Connection"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (status missing-prerequisite) (target "Connections::Connection"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/product_selection_unowned_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 17 1) (end 17 144))
      )
    )
  )
)
~~~
