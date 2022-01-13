use std::sync::{Arc, Mutex};
use std::fmt::{Display, Formatter, Error};
use actix_web::{web, web::ServiceConfig, HttpResponse, ResponseError};

use crate::{nodes, transaction, chain, requests, blockchain};

use nodes::RegisterError;
use transaction::Transaction;
use chain::ChainError;
use requests::RequestError;
use blockchain::Blockchain;

#[derive(Debug)]
pub enum ApiError {
    RegisterError(RegisterError),
    RequestError(RequestError),
    ChainError(ChainError),
}

impl Display for ApiError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self)
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::RegisterError(_) => HttpResponse::BadRequest().json(format!("Node registration error: {}", self)),
            ApiError::RequestError(_) => HttpResponse::BadRequest().json(format!("HTTP-request error: {}", self)),
            ApiError::ChainError(_) => HttpResponse::BadRequest().json(format!("Chain error: {}", self)),
        }
    }
}

pub fn initialize(service_config: &mut ServiceConfig) {
    service_config.route("/nodes", web::get().to(nodes));
    service_config.route("/nodes?url={url}", web::post().to(register_node));
    
    service_config.route("/chain", web::get().to(chain));
    service_config.route("/chain/block/{index}", web::get().to(block));
    service_config.route("/chain/resolve_conflicts", web::put().to(resolve_conflicts));
    
    service_config.route("/mine", web::post().to(mine));
    
    service_config.route("/current_transactions", web::get().to(current_transactions));
    service_config.route("/current_transactions?sender={sender}&recipient={recipient}&amount={amount}",
        web::post().to(add_transaction_to_current_transactions));
}

pub async fn nodes(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> Result<HttpResponse, ApiError> {
    let blockchain = match blockchain.lock() {
        Ok(blockchain) => blockchain,
        Err(_) => {
            return Err(ApiError::RegisterError(RegisterError::BlockchainLocked));
        }
    };
    
    Ok(HttpResponse::Ok().json(blockchain.nodes()))
}

pub async fn register_node(blockchain: web::Data<Arc<Mutex<Blockchain>>>, web::Path(url): web::Path<String>) ->
    Result<HttpResponse, ApiError> {
    let mut blockchain = match blockchain.lock() {
        Ok(blockchain) => blockchain,
        Err(_) => {
            return Err(ApiError::RegisterError(RegisterError::BlockchainLocked));
        }
    };
    
    let node = blockchain.register_node(url.as_str());
    
    match node {
        Ok(node) => Ok(HttpResponse::Ok().json(node)),
        Err(error) => Err(ApiError::RegisterError(error)),
    }
}

pub async fn chain(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> Result<HttpResponse, ApiError> {
    let blockchain = match blockchain.lock() {
        Ok(blockchain) => blockchain,
        Err(_) => {
            return Err(ApiError::RegisterError(RegisterError::BlockchainLocked));
        }
    };
    
    Ok(HttpResponse::Ok().json(blockchain.chain()))
}

pub async fn block(blockchain: web::Data<Arc<Mutex<Blockchain>>>, web::Path(index): web::Path<usize>) ->
    Result<HttpResponse, ApiError> {
     let blockchain = match blockchain.lock() {
        Ok(blockchain) => blockchain,
        Err(_) => {
            return Err(ApiError::RegisterError(RegisterError::BlockchainLocked));
        }
    };
    
    let block = blockchain.block(index);
    
    match block {
        Ok(block) => Ok(HttpResponse::Ok().json(block)),
        Err(error) => Err(ApiError::ChainError(error)),
    }
}

pub async fn resolve_conflicts(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> Result<HttpResponse, ApiError> {
    let mut blockchain = match blockchain.lock() {
        Ok(blockchain) => blockchain,
        Err(_) => {
            return Err(ApiError::RegisterError(RegisterError::BlockchainLocked));
        }
    };
    
    let result = blockchain.resolve_conflicts().await;
    
    match result {
        Ok(resolving_result) => Ok(HttpResponse::Ok().json(resolving_result)),
        Err(error) => Err(ApiError::RequestError(error)),
    }
}

pub async fn mine(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> Result<HttpResponse, ApiError> {
    let mut blockchain = match blockchain.lock() {
        Ok(blockchain) => blockchain,
        Err(_) => {
            return Err(ApiError::RegisterError(RegisterError::BlockchainLocked));
        }
    };
    
    Ok(HttpResponse::Ok().json(blockchain.mine()))
}

pub async fn current_transactions(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> Result<HttpResponse, ApiError> {
    let blockchain = match blockchain.lock() {
        Ok(blockchain) => blockchain,
        Err(_) => {
            return Err(ApiError::RegisterError(RegisterError::BlockchainLocked));
        }
    };
    
    Ok(HttpResponse::Ok().json(blockchain.current_transactions()))
}

pub async fn add_transaction_to_current_transactions(blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    web::Path(transaction): web::Path<Transaction>) -> Result<HttpResponse, ApiError> {
    let mut blockchain = match blockchain.lock() {
        Ok(blockchain) => blockchain,
        Err(_) => {
            return Err(ApiError::RegisterError(RegisterError::BlockchainLocked));
        }
    };
    
    Ok(HttpResponse::Ok().json(blockchain.add_transaction_to_current_transactions(transaction)))
}
