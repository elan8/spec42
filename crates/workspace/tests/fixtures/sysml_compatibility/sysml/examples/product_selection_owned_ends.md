# META
~~~ini
description=SysML Example (Association): ProductSelection_OwnedEnds
type=file
~~~
# SOURCE
~~~sysml
package ProductSelection_OwnedEnds_SysML {
	
	item def SelectionInfo;
	item def ShoppingCart;
	item def Product;
	
	// User-specified connection defiation definition
	connection def ProductSelection {
		item info: SelectionInfo;
		
		end [0..1] item cart: ShoppingCart[1];
		end [0..*] nonunique item selectedProduct: Product[1];
	}
	
	// Equivalent connection defiation definition with named end items.
	connection def ProductSelection1 {
		item info: SelectionInfo;
		
		end inCart[0..1] item cart: ShoppingCart[1];
		end selectedProducts[0..*] item selectedProduct: Product[1];
	}
	
	connection def SingleProductSelection specializes ProductSelection {
		end [0..1] item cart: ShoppingCart[1];
		end [0..1] item selectedProduct: Product[1];
	}

	connection def SingleProductSelection1 specializes ProductSelection1 {
		end inCart1 [0..1] item cart: ShoppingCart[1];
		end selectedProduct1 [0..1] item selectedProduct: Product[1];
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
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
LineComment,
KwConnection,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwConnection,KwDef,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
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
  (package_def 'ProductSelection_OwnedEnds_SysML'
    (item_def 'SelectionInfo')
    (item_def 'ShoppingCart')
    (item_def 'Product')
    (line_comment)
    (connection_def 'ProductSelection'
      (item_usage 'info' : 'SelectionInfo')
      (interface_end end 'cart' : 'ShoppingCart' multiplicity)
      (interface_end end 'selectedProduct' : 'Product' multiplicity nonunique))
    (line_comment)
    (connection_def 'ProductSelection1'
      (item_usage 'info' : 'SelectionInfo')
      (interface_end end 'inCart' : 'ShoppingCart' multiplicity)
      (interface_end end 'selectedProducts' : 'Product' multiplicity))
    (connection_def 'SingleProductSelection' :> 'ProductSelection'
      (interface_end end 'cart' : 'ShoppingCart' multiplicity)
      (interface_end end 'selectedProduct' : 'Product' multiplicity))
    (connection_def 'SingleProductSelection1' :> 'ProductSelection1'
      (interface_end end 'inCart1' : 'ShoppingCart' multiplicity)
      (interface_end end 'selectedProduct1' : 'Product' multiplicity))
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
package ProductSelection_OwnedEnds_SysML {
    item def SelectionInfo;
    item def ShoppingCart;
    item def Product;

    // User-specified connection defiation definition
    connection def ProductSelection {
        item info : SelectionInfo;

        end [0..1] cart : ShoppingCart;
        end [0..*] selectedProduct : Product nonunique;
    }

    // Equivalent connection defiation definition with named end items.
    connection def ProductSelection1 {
        item info : SelectionInfo;

        end [0..1] inCart : ShoppingCart;
        end [0..*] selectedProducts : Product;
    }

    connection def SingleProductSelection specializes ProductSelection {
        end [0..1] cart : ShoppingCart;
        end [0..1] selectedProduct : Product;
    }

    connection def SingleProductSelection1 specializes ProductSelection1 {
        end [0..1] inCart1 : ShoppingCart;
        end [0..1] selectedProduct1 : Product;
    }

    item def OnlineCustomer {
        item info1 : SelectionInfo;
        item myCart : ShoppingCart [1];
        item products : Product [0..*];

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
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))) (name "ProductSelection_OwnedEnds_SysML") (declared-name "ProductSelection_OwnedEnds_SysML")
      (contains
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer"))) (name "OnlineCustomer") (declared-name "OnlineCustomer"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::Product"))) (name "Product") (declared-name "Product"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection"))) (name "ProductSelection") (declared-name "ProductSelection")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (name "cart") (declared-name "cart") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::nonunique"))) (name "nonunique") (declared-name "nonunique") (declared (properties (end true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))))
          )
        )
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1"))) (name "ProductSelection1") (declared-name "ProductSelection1")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::inCart"))) (name "inCart") (declared-name "inCart") (declared (properties (end true)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::selectedProducts"))) (name "selectedProducts") (declared-name "selectedProducts") (declared (properties (end true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo"))) (name "SelectionInfo") (declared-name "SelectionInfo"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (name "ShoppingCart") (declared-name "ShoppingCart"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (name "SingleProductSelection") (declared-name "SingleProductSelection")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (name "cart") (declared-name "cart") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (name "selectedProduct") (declared-name "selectedProduct") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection")))))
          )
        )
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (name "SingleProductSelection1") (declared-name "SingleProductSelection1")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::inCart1"))) (name "inCart1") (declared-name "inCart1") (declared (properties (end true)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::selectedProduct1"))) (name "selectedProduct1") (declared-name "selectedProduct1") (declared (properties (end true)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1")))))
          )
        )
      )
    )
  )
  (relationships
    (connection (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (to (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::Product"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (to (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (to (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (to (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (to (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (to (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::Product"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
