//! Submódulo de `cpsock` (ADR-0026, etapa 2).

use super::*;

/// Limite de conexões simultâneas por origem (`per_ip = 0` desliga).
#[derive(Debug, Clone, Copy)]
pub struct ConnectionLimit {
    pub per_ip: usize,
}

impl ConnectionLimit {
    #[must_use]
    pub fn new(per_ip: usize) -> Self {
        Self { per_ip }
    }

    /// `WYD_MAX_CONNECTIONS_PER_IP` (padrão 6; `0` desliga). Multiboxing custa 4–5.
    #[must_use]
    pub fn from_env() -> Self {
        let per_ip = std::env::var("WYD_MAX_CONNECTIONS_PER_IP")
            .ok()
            .and_then(|valor| valor.trim().parse().ok())
            .unwrap_or(6);
        Self { per_ip }
    }
}

/// Vaga de conexão de um IP: libera a contagem no `Drop`, inclusive em pânico da thread.
pub(super) struct IpSlot {
    ativos: Option<Arc<std::sync::Mutex<std::collections::HashMap<std::net::IpAddr, usize>>>>,
    ip: std::net::IpAddr,
}

impl IpSlot {
    pub(super) fn reservar(
        ativos: &Arc<std::sync::Mutex<std::collections::HashMap<std::net::IpAddr, usize>>>,
        ip: std::net::IpAddr,
        maximo: usize,
    ) -> Option<Self> {
        if maximo == 0 {
            return Some(Self { ativos: None, ip });
        }
        let mut mapa = ativos.lock().unwrap_or_else(|e| e.into_inner());
        let atual = mapa.entry(ip).or_insert(0);
        if *atual >= maximo {
            return None;
        }
        *atual += 1;
        Some(Self {
            ativos: Some(Arc::clone(ativos)),
            ip,
        })
    }
}

impl Drop for IpSlot {
    fn drop(&mut self) {
        let Some(ativos) = &self.ativos else {
            return;
        };
        let mut mapa = ativos.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(atual) = mapa.get_mut(&self.ip) {
            *atual = atual.saturating_sub(1);
            if *atual == 0 {
                mapa.remove(&self.ip);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::net::{IpAddr, Ipv4Addr};
    use std::sync::{Arc, Mutex};

    fn mapa() -> Arc<Mutex<HashMap<IpAddr, usize>>> {
        Arc::new(Mutex::new(HashMap::new()))
    }

    fn ip(ultimo: u8) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, ultimo))
    }

    #[test]
    fn limite_por_ip_reserva_recusa_e_libera_no_drop() {
        let ativos = mapa();
        let primeiro = IpSlot::reservar(&ativos, ip(1), 2).expect("1a vaga");
        let segundo = IpSlot::reservar(&ativos, ip(1), 2).expect("2a vaga");
        assert!(
            IpSlot::reservar(&ativos, ip(1), 2).is_none(),
            "a terceira conexão do mesmo IP é recusada"
        );
        let outro = IpSlot::reservar(&ativos, ip(2), 2).expect("a contagem é por IP, não global");

        drop(segundo);
        let refeito = IpSlot::reservar(&ativos, ip(1), 2).expect("o Drop devolve a vaga");
        {
            let contagem = ativos.lock().expect("mapa sem pânico");
            assert_eq!(contagem.get(&ip(1)), Some(&2), "primeiro + refeito");
            assert_eq!(contagem.get(&ip(2)), Some(&1));
        }

        drop(refeito);
        drop(primeiro);
        drop(outro);
        assert!(
            ativos.lock().expect("mapa sem pânico").is_empty(),
            "soltar todas as vagas limpa o mapa"
        );
    }

    #[test]
    fn ultimo_drop_remove_o_ip_do_mapa() {
        let ativos = mapa();
        let slot = IpSlot::reservar(&ativos, ip(3), 1).expect("vaga única");
        drop(slot);
        assert!(
            ativos.lock().expect("mapa sem pânico").is_empty(),
            "zerar a contagem não deixa chave órfã"
        );
    }

    #[test]
    fn maximo_zero_desliga_o_limite() {
        let ativos = mapa();
        let slots: Vec<IpSlot> = (0..4)
            .map(|_| IpSlot::reservar(&ativos, ip(9), 0).expect("sem limite"))
            .collect();
        assert_eq!(slots.len(), 4);
        assert!(
            ativos.lock().expect("mapa sem pânico").is_empty(),
            "limite desligado não conta no mapa"
        );
        drop(slots);
    }

    #[test]
    fn from_env_le_a_variavel_ou_cai_no_default() {
        let var = "WYD_MAX_CONNECTIONS_PER_IP";
        std::env::set_var(var, " 9 ");
        assert_eq!(ConnectionLimit::from_env().per_ip, 9);
        std::env::set_var(var, "abc");
        assert_eq!(ConnectionLimit::from_env().per_ip, 6, "lixo cai no padrão");
        std::env::remove_var(var);
        assert_eq!(ConnectionLimit::from_env().per_ip, 6);
        assert_eq!(ConnectionLimit::new(0).per_ip, 0, "0 desliga");
    }
}
