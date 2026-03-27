//! Business logic for tickets.
use galvyn::rorm;
use galvyn::rorm::conditions::Condition;
use galvyn::rorm::db::Executor;
use galvyn::rorm::fields::types::MaxStr;
use galvyn::rorm::prelude::ForeignModelByField;
use time::OffsetDateTime;
use tracing::instrument;
use uuid::Uuid;

use crate::models::account::Account;
use crate::models::account::AccountUuid;
use crate::models::tickets::db::TicketModel;
use crate::models::tickets::db::TicketStatus;

pub(in crate::models) mod db;

/// Business model for tickets.
#[derive(Debug, Clone)]
pub struct Ticket {
    /// The ticket's UUID.
    pub uuid: TicketUuid,
    /// The account that created the ticket.
    pub created_by: Account,
    /// The account that the ticket is assigned to.
    pub assigned_to: Option<Account>,
    /// The timestamp of the ticket creation.
    pub timestamp: OffsetDateTime,
    /// The status of the ticket.
    pub status: TicketStatus,
    /// The title of the ticket.
    pub heading: MaxStr<255>,
    /// The body of the ticket.
    pub body: MaxStr<1024>,
    /// The response to the ticket.
    pub response: MaxStr<1024>,
}

/// A wrapper around a ticket UUID.
#[derive(Debug, Clone, Copy)]
pub struct TicketUuid(Uuid);

/// A request to create a ticket.
#[derive(Debug, Clone)]
pub struct CreateTicket {
    /// The account that created the ticket.
    pub created_by: AccountUuid,
    /// The account that the ticket is assigned to.
    pub assigned_to: Option<AccountUuid>,
    /// The title of the ticket.
    pub heading: MaxStr<255>,
    /// The body of the ticket.
    pub body: MaxStr<1024>,
}

impl Ticket {
    /// Retrieves all tickets from the database based on the account UUID of the user who created it.
    #[instrument(name = "Tickets::get_by_created_by", skip(db))]
    pub async fn get_by_created_by(
        db: impl Executor<'_>,
        account: AccountUuid,
    ) -> Result<Vec<Self>, rorm::Error> {
        Self::get_by_condition(db, TicketModel.created_by.equals(account.get_inner())).await
    }

    /// Retrieves all tickets from the database based on the account UUID of the user who is assigned to it.
    #[instrument(name = "Tickets::get_by_assigned_to", skip(db))]
    pub async fn get_by_assigned_to(
        db: impl Executor<'_>,
        account: AccountUuid,
    ) -> Result<Vec<Self>, rorm::Error> {
        Self::get_by_condition(
            db,
            TicketModel.assigned_to.equals(Some(account.get_inner())),
        )
        .await
    }

    /// Inserts a new `TicketModel` into the database
    #[instrument(name = "Tickets::create", skip(db))]
    pub async fn create(
        db: impl Executor<'_>,
        request: CreateTicket,
    ) -> Result<TicketUuid, rorm::Error> {
        let model = rorm::insert(db, TicketModel)
            .single(&TicketModel {
                uuid: Uuid::new_v4(),
                created_by: ForeignModelByField(request.created_by.get_inner()),
                assigned_to: request
                    .assigned_to
                    .map(|account| ForeignModelByField(account.get_inner())),
                timestamp: OffsetDateTime::now_utc(),
                status: TicketStatus::Open,
                heading: request.heading,
                body: request.body,
                response: MaxStr::default(),
            })
            .await?;
        Ok(TicketUuid(model.uuid))
    }

    /// Updates the status of a ticket.
    #[instrument(name = "Tickets::update_status", skip(db))]
    pub async fn update_status(
        &mut self,
        db: impl Executor<'_>,
        status: TicketStatus,
    ) -> Result<(), rorm::Error> {
        self.status = status.clone();
        rorm::update(db, TicketModel)
            .set(TicketModel.status, status)
            .condition(TicketModel.uuid.equals(self.uuid.0))
            .await?;
        Ok(())
    }

    /// Updates the response of a ticket.
    #[instrument(name = "Tickets::update_response", skip(db))]
    pub async fn update_response(
        &mut self,
        db: impl Executor<'_>,
        response: MaxStr<1024>,
    ) -> Result<(), rorm::Error> {
        self.response = response.clone();
        rorm::update(db, TicketModel)
            .set(TicketModel.response, response)
            .condition(TicketModel.uuid.equals(self.uuid.0))
            .await?;
        Ok(())
    }

    /// Updates the assigned to field of a ticket.
    #[instrument(name = "Tickets::update_assigned_to", skip(db))]
    pub async fn update_assigned_to(
        &mut self,
        db: impl Executor<'_>,
        account: Account,
    ) -> Result<(), rorm::Error> {
        self.assigned_to = Some(account.clone());
        rorm::update(db, TicketModel)
            .set(
                TicketModel.assigned_to,
                Some(ForeignModelByField(account.uuid.get_inner())),
            )
            .condition(TicketModel.uuid.equals(self.uuid.0))
            .await?;
        Ok(())
    }

    /// Asynchronously retrieves a vector of objects based on a specified condition from the database.
    ///
    /// This function performs a query on the `TicketModel` table using the provided condition and
    /// retrieves all matching records. Each record is then transformed into an instance of the
    /// implementing type using the `Self::new` constructor before being added to the result vector.
    pub async fn get_by_condition(
        db: impl Executor<'_>,
        condition: impl Condition<'_>,
    ) -> Result<Vec<Self>, rorm::Error> {
        let mut guard = db.ensure_transaction().await?;
        let models = rorm::query(guard.get_transaction(), TicketModel)
            .condition(condition)
            .all()
            .await?;

        let mut result = vec![];
        for model in models {
            result.push(Self::new(guard.get_transaction(), model).await?);
        }
        Ok(result)
    }

    /// Creates a new `Ticket` instance from a `TicketModel`.
    pub async fn new(db: impl Executor<'_>, model: TicketModel) -> Result<Self, rorm::Error> {
        let mut guard = db.ensure_transaction().await?;
        let assigned_to = match model.assigned_to {
            Some(model) => Some(Account::from(model.query(guard.get_transaction()).await?)),
            None => None,
        };

        let value = Self {
            uuid: TicketUuid(model.uuid),
            created_by: Account::from(model.created_by.query(guard.get_transaction()).await?),
            assigned_to,
            timestamp: model.timestamp,
            status: model.status,
            heading: model.heading,
            body: model.body,
            response: model.response,
        };

        guard.commit().await?;
        Ok(value)
    }
}
