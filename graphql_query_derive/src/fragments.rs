use graphql_parser::query::SelectionSet;
use proc_macro2::{Ident, Span, TokenStream};
use query::QueryContext;

#[derive(Debug, PartialEq)]
pub struct GqlFragment {
    pub name: String,
    pub on: String,
    pub selection: SelectionSet,
}

impl GqlFragment {
    pub fn to_rust(&self, context: &QueryContext) -> TokenStream {
        let name_ident = Ident::new(&self.name, Span::call_site());
        let object = context.schema.objects.get(&self.on).expect("oh, noes");
        let field_impls = object
            .field_impls_for_selection(context, &self.selection, &self.name)
            .unwrap();
        let fields = object.response_fields_for_selection(context, &self.selection, &self.name);

        quote!{
            #[derive(Debug, Deserialize, Serialize)]
            pub struct #name_ident {
                #(#fields,)*
            }

            #(#field_impls)*
        }
    }
}
