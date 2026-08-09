# META
~~~ini
description=Standard Library: Domain Libraries/Metadata/ImageMetadata
type=file
~~~
# SOURCE
~~~sysml
standard library package ImageMetadata {
	doc
	/*
	 * This package provides attributive data and metadata to allow a model element to be
	 * annotated with an image to be used in its graphical rendering or as a marker to
	 * adorn graphical or textual renderings.
	 */
	 
	private import ScalarValues::String;
	
	attribute def Image {
		doc
		/*
		 * Image provides the data necessary for the physical definition of 
		 * a graphical image.
		 */
		 
		attribute content : String[0..1] {
			doc
			/*
			 * Binary data for the image according to the given MIME type, 
			 * encoded as given by the encoding.
			 */
		}
		
		attribute encoding : String[0..1] {
			doc
			/*
			 * Describes how characters in the content are to be decoded into 
			 * binary data. At least "base64", "hex", "identify", and "JSONescape"
			 * shall be supported.
			 */
		}
		
		attribute type : String[0..1] {
			doc
			/*
			 * The MIME type according to which the content should be interpreted.
			 */
		}
		
		attribute location : String[0..1] {
			doc
			/*
			 * A URI for the location of a resource containing the image content,
			 * as an alternative for embedding it in the content attribute.
			 */
		}
	}
	
	metadata def Icon {
		doc
		/*
		 * Icon metadata can be used to annotate a model element with an image to be used
		 * to show render the element on a diagram and/or a small image to be used as an
		 * adornment on a graphical or textual rendering. Alternatively, another metadata
		 * definition can be annotated with an Icon to indicate that any model element 
		 * annotated by the containing metadata can be rendered according to the Icon.
		 */
		 
		attribute fullImage : Image[0..1] {
			doc
			/*
			 * A full-sized image that can be used to render the annotated element on a
			 * graphical view, potentially as an alternative to its standard rendering.
			 */
		}
		
		attribute smallImage : Image[0..1] {
			doc
			/*
			 * A smaller image that can be used as an adornment on the graphical rendering
			 * of the annotated element or as a marker in a textual rendering.
			 */
		}
	}
	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,KwType,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ImageMetadata'
    (documentation)
    (import_decl private 'ScalarValues::String')
    (attribute_def 'Image'
      (documentation)
      (attribute_usage 'content' : 'String' multiplicity
        (documentation))
      (attribute_usage 'encoding' : 'String' multiplicity
        (documentation))
      (attribute_usage 'type' : 'String' multiplicity
        (documentation))
      (attribute_usage 'location' : 'String' multiplicity
        (documentation)))
    (metadata_def 'Icon'
      (documentation)
      (attribute_usage 'fullImage' : 'Image' multiplicity
        (documentation))
      (attribute_usage 'smallImage' : 'Image' multiplicity
        (documentation)))))
~~~
# FORMAT
~~~sysml
standard library package ImageMetadata {
    doc
    /*
	 * This package provides attributive data and metadata to allow a model element to be
	 * annotated with an image to be used in its graphical rendering or as a marker to
	 * adorn graphical or textual renderings.
	 */

    private import ScalarValues::String;

    attribute def Image {
        doc
        /*
		 * Image provides the data necessary for the physical definition of 
		 * a graphical image.
		 */

        attribute content : String[0..1] {
            doc
            /*
			 * Binary data for the image according to the given MIME type, 
			 * encoded as given by the encoding.
			 */
        }

        attribute encoding : String[0..1] {
            doc
            /*
			 * Describes how characters in the content are to be decoded into 
			 * binary data. At least "base64", "hex", "identify", and "JSONescape"
			 * shall be supported.
			 */
        }

        attribute type : String[0..1] {
            doc
            /*
			 * The MIME type according to which the content should be interpreted.
			 */
        }

        attribute location : String[0..1] {
            doc
            /*
			 * A URI for the location of a resource containing the image content,
			 * as an alternative for embedding it in the content attribute.
			 */
        }
    }

    metadata def Icon {
        doc
        /*
		 * Icon metadata can be used to annotate a model element with an image to be used
		 * to show render the element on a diagram and/or a small image to be used as an
		 * adornment on a graphical or textual rendering. Alternatively, another metadata
		 * definition can be annotated with an Icon to indicate that any model element 
		 * annotated by the containing metadata can be rendered according to the Icon.
		 */

        attribute fullImage : Image[0..1] {
            doc
            /*
			 * A full-sized image that can be used to render the annotated element on a
			 * graphical view, potentially as an alternative to its standard rendering.
			 */
        }

        attribute smallImage : Image[0..1] {
            doc
            /*
			 * A smaller image that can be used as an adornment on the graphical rendering
			 * of the annotated element or as a marker in a textual rendering.
			 */
        }
    }

}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ImageMetadata"))) (name "ImageMetadata") (declared-name "ImageMetadata")
      (contains
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ImageMetadata::Icon"))) (name "Icon") (declared-name "Icon")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::Icon::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Icon")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (name "fullImage") (declared-name "fullImage") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Icon"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (name "smallImage") (declared-name "smallImage") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Icon"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image")))))
              )
            )
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ImageMetadata::Image"))) (name "Image") (declared-name "Image") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::Image::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (name "content") (declared-name "content") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::Image::content::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (name "encoding") (declared-name "encoding") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::Image::encoding::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (name "location") (declared-name "location") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::Image::location::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (name "type") (declared-name "type") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::Image::type::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ImageMetadata::Image")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ImageMetadata::String"))) (name "String") (declared-name "String"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ImageMetadata::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Icon::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata::Icon"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Image::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata::Image"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Image::content::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata::Image::content"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Image::encoding::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Image::location::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata::Image::location"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Image::type::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata::Image::type"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::_documentation"))) (to (node (document "d0") (qualified-name "ImageMetadata"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (to (node (document "d0") (qualified-name "ImageMetadata::Image"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (to (node (document "d0") (qualified-name "ImageMetadata::Image"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/image_metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 36))
      )
    )
  )
)
~~~
