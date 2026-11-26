"""Camada única de verificação do site e do planejamento.

O pacote concentra o índice de arquivos, o cache de leitura e os subcomandos do
gate. A regra de cada domínio continua nos módulos de `checks/`; este pacote só
orquestra, compartilha a árvore e formata a saída para o agente.
"""

from .workspace import Workspace

__all__ = ["Workspace"]
