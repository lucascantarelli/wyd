//! `wyd-protocol` — pacotes, opcodes, offsets e goldens do fio.
//!
//! Estrutura inicial criada em 2026-11-26 para o cenário real de desenvolvimento.
//! A fronteira, os módulos e o critério de aceite são do plano do sistema em
//! `planejamento-1.0.0/wyd-protocol/`; nenhum byte, opcode ou transformação está
//! implementado aqui. Cada módulo abaixo é uma reserva nomeada pelo plano e ganha
//! código só com a página dona aceita (`AGENTS.md` §5, item 1).

#![forbid(unsafe_code)]

pub mod codec;
pub mod error;
pub mod generated;
pub mod payload;
pub mod wire;
