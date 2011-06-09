#define GI_NIL "Nil"
#define GI_CLASS "Class"
#define GI_ARRAY "Array"
#define GI_STRING "String"
#define GI_FILE "File"
#define GI_EXCEPTION "Exception"
#define GI_INTEGER "Integer"
#define GI_ENGINE "Engine"
#define GI_FUNCTION "Function"

#define ENGINE boost::dynamic_pointer_cast<giEngine>(g_engine)

#define _c(class_name) ENGINE->lookup_class(class_name)

#define NIL boost::dynamic_pointer_cast<giNil>(_c(GI_NIL))->instance()
#define EXCEPTION boost::dynamic_pointer_cast<giException>(_c(GI_EXCEPTION))
