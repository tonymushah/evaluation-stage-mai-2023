use async_graphql::{ComplexObject, SimpleObject};
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::models::{materiel::Materiel, unite::Unite, v_devis_materiel::VDevisMateriel};

#[derive(Debug, SimpleObject, Clone)]
#[graphql(complex)]
pub struct ClientDevisItem {
    pub id: Uuid,
    #[graphql(skip)]
    pub materiel_id: String,
    #[graphql(skip)]
    pub materiel: String,
    pub quantite: BigDecimal,
    #[graphql(skip)]
    pub prix_unitaire: BigDecimal,
    #[graphql(skip)]
    pub unite: String,
    #[graphql(skip)]
    pub unite_id: Uuid,
    pub prix_total: BigDecimal,
}

impl From<VDevisMateriel> for ClientDevisItem {
    fn from(value: VDevisMateriel) -> Self {
        Self {
            id: value.id_devis,
            materiel_id: value.materiel_id,
            materiel: value.materiel,
            quantite: value.quantite,
            prix_unitaire: value.prix_unitaire,
            unite: value.unite,
            unite_id: value.unite_id,
            prix_total: value.prix_total,
        }
    }
}

#[ComplexObject]
impl ClientDevisItem {
    pub async fn materiel(&self) -> Materiel {
        Materiel {
            code: self.materiel_id.clone(),
            designation: self.materiel.clone(),
            prix_unitaire: self.prix_unitaire.clone(),
            unite_id: self.unite_id,
        }
    }
    pub async fn unite(&self) -> Unite {
        Unite {
            id_unite: self.unite_id,
            designation: self.unite.clone(),
        }
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct ClientDevis {
    pub items: Vec<ClientDevisItem>,
}

#[ComplexObject]
impl ClientDevis {
    pub async fn prix_total(&self) -> Option<BigDecimal> {
        self.items
            .iter()
            .map(|i| i.prix_total.clone())
            .reduce(|acc, e| acc + e)
    }
}
