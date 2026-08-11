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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "image_metadata.md"
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "18e4e734c7d7ba1946daab05e6a58be06e9ca96f3a023410fb3bc824171fc676") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ImageMetadata"))) (kind "package") (name "ImageMetadata") (declared-name "ImageMetadata") (range (start (line 0) (character 0)) (end (line 0) (character 2099))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon"))) (kind "metadata def") (name "Icon") (declared-name "Icon") (range (start (line 50) (character 1)) (end (line 50) (character 896))) (parent (node (document "d0") (qualified-name "ImageMetadata"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::_documentation"))) (kind "documentation") (name "") (range (start (line 50) (character 1)) (end (line 50) (character 896))) (parent (node (document "d0") (qualified-name "ImageMetadata::Icon"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind "attribute") (name "fullImage") (declared-name "fullImage") (range (start (line 60) (character 2)) (end (line 60) (character 219))) (parent (node (document "d0") (qualified-name "ImageMetadata::Icon"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage::_documentation"))) (kind "documentation") (name "") (range (start (line 60) (character 2)) (end (line 60) (character 219))) (parent (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind "attribute") (name "smallImage") (declared-name "smallImage") (range (start (line 68) (character 2)) (end (line 68) (character 214))) (parent (node (document "d0") (qualified-name "ImageMetadata::Icon"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image") (range none)))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage::_documentation"))) (kind "documentation") (name "") (range (start (line 68) (character 2)) (end (line 68) (character 214))) (parent (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image"))) (kind "attribute def") (name "Image") (declared-name "Image") (range (start (line 10) (character 1)) (end (line 10) (character 884))) (parent (node (document "d0") (qualified-name "ImageMetadata"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::_documentation"))) (kind "documentation") (name "") (range (start (line 10) (character 1)) (end (line 10) (character 884))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (kind "attribute") (name "content") (declared-name "content") (range (start (line 17) (character 2)) (end (line 17) (character 167))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::content::_documentation"))) (kind "documentation") (name "") (range (start (line 17) (character 2)) (end (line 17) (character 167))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image::content"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (kind "attribute") (name "encoding") (declared-name "encoding") (range (start (line 25) (character 2)) (end (line 25) (character 231))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::encoding::_documentation"))) (kind "documentation") (name "") (range (start (line 25) (character 2)) (end (line 25) (character 231))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (kind "attribute") (name "location") (declared-name "location") (range (start (line 41) (character 2)) (end (line 41) (character 201))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::location::_documentation"))) (kind "documentation") (name "") (range (start (line 41) (character 2)) (end (line 41) (character 201))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image::location"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (kind "attribute") (name "type") (declared-name "type") (range (start (line 34) (character 2)) (end (line 34) (character 131))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::type::_documentation"))) (kind "documentation") (name "") (range (start (line 34) (character 2)) (end (line 34) (character 131))) (parent (node (document "d0") (qualified-name "ImageMetadata::Image::type"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 8) (character 1)) (end (line 8) (character 37))) (parent (node (document "d0") (qualified-name "ImageMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2099))) (parent (node (document "d0") (qualified-name "ImageMetadata"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 8) (character 16)) (end (line 8) (character 36))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (target (node (document "d0") (qualified-name "ImageMetadata::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (target (node (document "d0") (qualified-name "ImageMetadata::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (target (node (document "d0") (qualified-name "ImageMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (target (node (document "d0") (qualified-name "ImageMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (target (node (document "d0") (qualified-name "ImageMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (target (node (document "d0") (qualified-name "ImageMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
