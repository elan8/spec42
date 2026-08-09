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
(model
  (namespace
    (package 'ProductSelection_OwnedEnds_SysML'
      (item_def 'SelectionInfo')
      (item_def 'ShoppingCart')
      (item_def 'Product')
      (connection_def 'ProductSelection'
        (item_usage composite 'info' : 'ProductSelection_OwnedEnds_SysML::SelectionInfo'[item_def])
        (port_usage end 'cart' : 'ProductSelection_OwnedEnds_SysML::ShoppingCart'[item_def]
          (multiplicity_range [0..1]))
        (port_usage end 'selectedProduct' : 'ProductSelection_OwnedEnds_SysML::Product'[item_def]
          (multiplicity_range [0..*])))
      (connection_def 'ProductSelection1'
        (item_usage composite 'info' : 'ProductSelection_OwnedEnds_SysML::SelectionInfo'[item_def])
        (port_usage end 'inCart' : 'ProductSelection_OwnedEnds_SysML::ShoppingCart'[item_def]
          (multiplicity_range [0..1]))
        (port_usage end 'selectedProducts' : 'ProductSelection_OwnedEnds_SysML::Product'[item_def]
          (multiplicity_range [0..*])))
      (connection_def 'SingleProductSelection' :> 'ProductSelection_OwnedEnds_SysML::ProductSelection'[connection_def]
        (port_usage end 'cart' : 'ProductSelection_OwnedEnds_SysML::ShoppingCart'[item_def] :>> 'ProductSelection_OwnedEnds_SysML::ProductSelection::cart'[port_usage][implied]
          (multiplicity_range [0..1]))
        (port_usage end 'selectedProduct' : 'ProductSelection_OwnedEnds_SysML::Product'[item_def] :>> 'ProductSelection_OwnedEnds_SysML::ProductSelection::selectedProduct'[port_usage][implied]
          (multiplicity_range [0..1])))
      (connection_def 'SingleProductSelection1' :> 'ProductSelection_OwnedEnds_SysML::ProductSelection1'[connection_def]
        (port_usage end 'inCart1' : 'ProductSelection_OwnedEnds_SysML::ShoppingCart'[item_def] :>> 'ProductSelection_OwnedEnds_SysML::ProductSelection1::inCart'[port_usage][implied]
          (multiplicity_range [0..1]))
        (port_usage end 'selectedProduct1' : 'ProductSelection_OwnedEnds_SysML::Product'[item_def] :>> 'ProductSelection_OwnedEnds_SysML::ProductSelection1::selectedProducts'[port_usage][implied]
          (multiplicity_range [0..1])))
      (item_def 'OnlineCustomer'
        (item_usage composite 'info1' : 'ProductSelection_OwnedEnds_SysML::SelectionInfo'[item_def])
        (item_usage composite 'myCart' : 'ProductSelection_OwnedEnds_SysML::ShoppingCart'[item_def]
          (multiplicity_range [1]))
        (item_usage composite 'products' : 'ProductSelection_OwnedEnds_SysML::Product'[item_def]
          (multiplicity_range [0..*]))
        (connection_usage composite 'ps1' : 'ProductSelection_OwnedEnds_SysML::ProductSelection'[connection_def]
          (connector_end 'myCart')
          (connector_end 'products')
          (reference_usage reference :>> 'ProductSelection_OwnedEnds_SysML::ProductSelection::info'[item_usage]
            (feature_value (=))))
        (connection_usage composite 'ps2' : 'ProductSelection_OwnedEnds_SysML::ProductSelection'[connection_def]
          (connector_end 'myCart')
          (connector_end 'products')
          (reference_usage reference :>> 'ProductSelection_OwnedEnds_SysML::ProductSelection::info'[item_usage]
            (feature_value (=))))))))
~~~
