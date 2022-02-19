/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
//! Error-handling utilities
use std::borrow::Cow;

use db_core::dev::*;
use sqlx::Error;

/// map postgres errors to [DBError](DBError) types
pub fn map_register_err(e: Error) -> DBError {
    if let Error::Database(err) = e {
        if err.code() == Some(Cow::from("23505")) {
            let msg = err.message();
            if msg.contains("gists_users_username_key") {
                DBError::DuplicateUsername
            } else if msg.contains("gists_users_email_key") {
                DBError::DuplicateEmail
            } else if msg.contains("gists_users_secret_key") {
                DBError::DuplicateSecret
            } else if msg.contains("gists_gists_public_id") {
                DBError::GistIDTaken
            } else {
                DBError::DBError(Box::new(Error::Database(err)))
            }
        } else {
            DBError::DBError(Box::new(Error::Database(err)))
        }
    } else {
        DBError::DBError(Box::new(e))
    }
}
