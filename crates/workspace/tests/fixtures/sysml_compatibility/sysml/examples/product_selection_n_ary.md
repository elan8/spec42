# META
~~~ini
description=SysML Example (Association): ProductSelection_N_ary
type=file
~~~
# SOURCE
~~~sysml
package ProductSelection_N_ary_SysML {
	
	item def ShoppingCart;
	item def Product;
	item def Account;
	
	// User-specified connection defiation definition
	connection def ProductSelection {
		end [0..1] item cart: ShoppingCart[1];
		end [0..*] item selectedProduct: Product[1];
		end [1..1] item account : Account[1];
	}
	
	// Equivalent connection defiation definition with named end items.
	connection def ProductSelection1 {
		end inCart[0..1] item cart: ShoppingCart[1];
		end selectedProducts[0..*] item selectedProduct: Product[1];
		end withAccount[1..1] item account : Account[1];
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
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ProductSelection_N_ary_SysML'
    (item_def 'ShoppingCart')
    (item_def 'Product')
    (item_def 'Account')
    (line_comment)
    (connection_def 'ProductSelection'
      (interface_end end 'cart' : 'ShoppingCart' multiplicity)
      (interface_end end 'selectedProduct' : 'Product' multiplicity)
      (interface_end end 'account' : 'Account' multiplicity))
    (line_comment)
    (connection_def 'ProductSelection1'
      (interface_end end 'inCart' : 'ShoppingCart' multiplicity)
      (interface_end end 'selectedProducts' : 'Product' multiplicity)
      (interface_end end 'withAccount' : 'Account' multiplicity))))
~~~
# FORMAT
~~~sysml
package ProductSelection_N_ary_SysML {
    item def ShoppingCart;
    item def Product;
    item def Account;

    // User-specified connection defiation definition
    connection def ProductSelection {
        end [0..1] cart : ShoppingCart;
        end [0..*] selectedProduct : Product;
        end [1..1] account : Account;
    }

    // Equivalent connection defiation definition with named end items.
    connection def ProductSelection1 {
        end [0..1] inCart : ShoppingCart;
        end [0..*] selectedProducts : Product;
        end [1..1] withAccount : Account;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML"))) (name "ProductSelection_N_ary_SysML") (declared-name "ProductSelection_N_ary_SysML")
      (contains
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (name "Account") (declared-name "Account"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (name "Product") (declared-name "Product"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (name "ProductSelection") (declared-name "ProductSelection")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (name "account") (declared-name "account") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (name "cart") (declared-name "cart") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (name "selectedProduct") (declared-name "selectedProduct") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection")))))
          )
        )
        (element (kind "connection def") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (name "ProductSelection1") (declared-name "ProductSelection1")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::inCart"))) (name "inCart") (declared-name "inCart") (declared (properties (end true)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProducts"))) (name "selectedProducts") (declared-name "selectedProducts") (declared (properties (end true)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::withAccount"))) (name "withAccount") (declared-name "withAccount") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (name "ShoppingCart") (declared-name "ShoppingCart"))
      )
    )
  )
  (relationships
    (connection (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (to (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Account"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (to (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Product"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (to (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Account"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (to (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (to (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Product"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
