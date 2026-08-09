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
        item selectedProducts : Product [0..*];
    }
    item def Product {
        item inCart : ShoppingCart [0..1];
    }

    connection def ProductSelection {
        item info : SelectionInfo [1];

        crosses selectedProduct.inCart;
        crosses cart.selectedProducts;
    }

    connection def SingleProductSelection :> ProductSelection {
        end cart : ShoppingCart;
        end [0..1] selectedProduct : Product;
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
(model
  (namespace
    (package 'ProductSelection_UnownedEnds_SysML'
      (item_def 'SelectionInfo')
      (item_def 'ShoppingCart'
        (item_usage composite 'selectedProducts' : 'ProductSelection_UnownedEnds_SysML::Product'[item_def]
          (multiplicity_range [0..*])))
      (item_def 'Product'
        (item_usage composite 'inCart' : 'ProductSelection_UnownedEnds_SysML::ShoppingCart'[item_def]
          (multiplicity_range [0..1])))
      (connection_def 'ProductSelection'
        (item_usage composite 'info' : 'ProductSelection_UnownedEnds_SysML::SelectionInfo'[item_def]
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (not_implemented 'malformed'))
      (connection_def 'SingleProductSelection' :> 'ProductSelection_UnownedEnds_SysML::ProductSelection'[connection_def]
        (port_usage end 'cart' : 'ProductSelection_UnownedEnds_SysML::ShoppingCart'[item_def])
        (port_usage end 'selectedProduct' : 'ProductSelection_UnownedEnds_SysML::Product'[item_def]
          (multiplicity_range [0..1])))
      (item_def 'OnlineCustomer'
        (item_usage composite 'info1' : 'ProductSelection_UnownedEnds_SysML::SelectionInfo'[item_def])
        (item_usage composite 'myCart' : 'ProductSelection_UnownedEnds_SysML::ShoppingCart'[item_def]
          (multiplicity_range [1]))
        (item_usage composite 'products' : 'ProductSelection_UnownedEnds_SysML::Product'[item_def]
          (multiplicity_range [0..*]))
        (connection_usage composite 'ps1' : 'ProductSelection_UnownedEnds_SysML::ProductSelection'[connection_def]
          (connector_end 'myCart')
          (connector_end 'products')
          (reference_usage reference :>> 'ProductSelection_UnownedEnds_SysML::ProductSelection::info'[item_usage]
            (feature_value (=))))
        (connection_usage composite 'ps2' : 'ProductSelection_UnownedEnds_SysML::ProductSelection'[connection_def]
          (connector_end 'myCart')
          (connector_end 'products')
          (reference_usage reference :>> 'ProductSelection_UnownedEnds_SysML::ProductSelection::info'[item_usage]
            (feature_value (=))))))))
~~~
