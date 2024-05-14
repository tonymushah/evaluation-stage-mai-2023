use actix_web::web::block;
use async_graphql::{ComplexObject, Context, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use time::Date;
use uuid::Uuid;

use crate::{
    models::{
        finition::Finition, type_chantier::TypeChantier, v_chantier_finition::VChantierFinition,
        v_devis_materiel::VDevisMateriel,
    },
    DbPool,
};

use super::devis::{ClientDevis, ClientDevisItem};

#[derive(Debug, SimpleObject, Clone)]
#[graphql(complex)]
pub struct ClientChantier {
    pub id: Uuid,
    pub date_debut: Option<Date>,
    #[graphql(skip)]
    pub finition_prix: BigDecimal,
    #[graphql(skip)]
    pub finition_duree: BigDecimal,
    #[graphql(skip)]
    pub finition: String,
    #[graphql(skip)]
    pub id_finition: Uuid,
    #[graphql(skip)]
    pub type_chantier_id: Uuid,
}

impl From<VChantierFinition> for ClientChantier {
    fn from(value: VChantierFinition) -> Self {
        Self {
            id: value.id_chantier,
            date_debut: value.date_debut,
            finition_prix: value.finition_prix,
            finition_duree: value.finition_duree,
            finition: value.finition,
            id_finition: value.id_finition,
            type_chantier_id: value.type_chantier_id,
        }
    }
}

#[ComplexObject]
impl ClientChantier {
    pub async fn finition(&self) -> Finition {
        Finition {
            id_finition: self.id_finition,
            designation: self.finition.clone(),
            prix: self.finition_prix.clone(),
            duree: self.finition_duree.clone(),
            is_standard: None,
        }
    }
    pub async fn type_chantier(&self, ctx: &Context<'_>) -> crate::Result<TypeChantier> {
        let mut pool = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?.get()?;
        let to_use_id = self.type_chantier_id;
        block(move || {
            use crate::schema::type_chantier::dsl::*;
            let res = type_chantier
                .filter(id_type_chantier.eq(to_use_id))
                .limit(1)
                .select(TypeChantier::as_select())
                .load(&mut pool)?
                .first()
                .ok_or(diesel::result::Error::NotFound)?
                .clone();
            Ok::<TypeChantier, crate::Error>(res)
        })
        .await?
    }
    pub async fn devis(&self, ctx: &Context<'_>) -> crate::Result<ClientDevis> {
        let mut pool = ctx.data::<DbPool>().map_err(crate::Error::GraphQL)?.get()?;
        let to_use_id = self.type_chantier_id;
        block(move || {
            use crate::views::v_devis_materiel::dsl::*;
            let res: Vec<VDevisMateriel> = v_devis_materiel
                .filter(type_chantier.eq(to_use_id))
                .select(VDevisMateriel::as_select())
                .load(&mut pool)?;
            Ok(ClientDevis {
                items: res
                    .into_iter()
                    .map(|i| -> ClientDevisItem { i.into() })
                    .collect(),
            })
        })
        .await?
    }
}
