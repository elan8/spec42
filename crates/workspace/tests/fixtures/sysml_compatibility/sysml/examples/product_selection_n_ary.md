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
(model
  (namespace
    (package 'ProductSelection_N_ary_SysML'
      (item_def 'ShoppingCart')
      (item_def 'Product')
      (item_def 'Account')
      (connection_def 'ProductSelection'
        (port_usage end 'cart' : 'ProductSelection_N_ary_SysML::ShoppingCart'[item_def]
          (multiplicity_range [0..1]))
        (port_usage end 'selectedProduct' : 'ProductSelection_N_ary_SysML::Product'[item_def]
          (multiplicity_range [0..*]))
        (port_usage end 'account' : 'ProductSelection_N_ary_SysML::Account'[item_def]
          (multiplicity_range [1..1])))
      (connection_def 'ProductSelection1'
        (port_usage end 'inCart' : 'ProductSelection_N_ary_SysML::ShoppingCart'[item_def]
          (multiplicity_range [0..1]))
        (port_usage end 'selectedProducts' : 'ProductSelection_N_ary_SysML::Product'[item_def]
          (multiplicity_range [0..*]))
        (port_usage end 'withAccount' : 'ProductSelection_N_ary_SysML::Account'[item_def]
          (multiplicity_range [1..1]))))))
~~~
