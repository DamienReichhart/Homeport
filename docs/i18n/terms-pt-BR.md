# Homeport glossary — pt-BR

Register: "você" (as Windows/macOS pt-BR). Buttons and menu items in the infinitive
("Remover", "Salvar", "Reimplantar"), as Windows pt-BR does. Toasts/status in the past participle
("Removido.", "Copiado"). No exclamation marks.

| English term                                  | pt-BR term                                     | Grammatical notes                                                    | Words to avoid                                   |
| --------------------------------------------- | ---------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------ |
| Host                                          | host                                           | masc. (o host, os hosts); capitalised only where English capitalises | servidor, instância, endpoint, contexto, máquina |
| Engine                                        | Engine                                         | masc. (o Engine); kept in English like "Docker Engine"               | daemon, servidor, motor, mecanismo               |
| Engine kind                                   | tipo de Engine                                 | masc.                                                                | —                                                |
| Instance (Incus)                              | instância                                      | fem.                                                                 | VM (for both), contêiner Incus                   |
| Project (Incus)                               | projeto                                        | masc.                                                                | —                                                |
| Profile                                       | perfil                                         | masc.                                                                | —                                                |
| Pod                                           | Pod (Kubernetes); pod do Podman                | masc.                                                                | grupo                                            |
| Quadlet unit                                  | unidade Quadlet                                | fem.                                                                 | arquivo Quadlet (for the unit)                   |
| Bastion                                       | bastion                                        | masc. (o bastion, os bastions)                                       | jump host, proxy, gateway, salto                 |
| Swarm                                         | Swarm                                          | masc. (o Swarm)                                                      | cluster (alone), modo swarm                      |
| Cluster                                       | cluster                                        | masc. (o cluster)                                                    | contexto, k8s, ambiente                          |
| Port forward                                  | encaminhamento de porta                        | masc.; pl. encaminhamentos de porta                                  | túnel, proxy, porta publicada                    |
| Release (Helm)                                | release                                        | fem. (a release, as releases)                                        | implantação, app, instalação                     |
| Chart repository                              | repositório de charts                          | masc.                                                                | repo Helm, registro                              |
| Node (Swarm)                                  | nó                                             | masc. (o nó, os nós); Kubernetes Node stays "Node"                   | membro, máquina, servidor                        |
| Service (Swarm)                               | serviço                                        | masc.; Kubernetes Service stays "Service"                            | app, implantação                                 |
| Task (Swarm / containerd)                     | tarefa                                         | fem.                                                                 | réplica, job                                     |
| Board                                         | Quadro                                         | masc. (o Quadro)                                                     | painel, dashboard                                |
| Command palette                               | paleta de comandos                             | fem.                                                                 | —                                                |
| Stack                                         | stack                                          | fem. (a stack, as stacks)                                            | compose, projeto, aplicação                      |
| Managed Stack                                 | stack gerenciada                               | fem.                                                                 | stack própria, stack local                       |
| Linked Stack                                  | stack vinculada                                | fem.                                                                 | stack local, stack de pasta                      |
| Synced Stack                                  | stack sincronizada                             | fem.                                                                 | stack git, stack GitOps                          |
| External Stack                                | stack externa                                  | fem.                                                                 | stack não gerenciada, stack estrangeira          |
| Repository                                    | repositório                                    | masc.                                                                | repo                                             |
| Deploy key                                    | chave de deploy                                | fem.                                                                 | chave SSH (alone), chave de acesso               |
| Adopt                                         | adotar / adoção                                | verb / fem. noun                                                     | importar, reivindicar                            |
| Registry login                                | login de registro                              | masc.                                                                | credencial, conta                                |
| Helper container                              | contêiner auxiliar                             | masc.                                                                | sidecar, agente                                  |
| Container                                     | contêiner                                      | masc.; pl. contêineres                                               | container                                        |
| Image                                         | imagem                                         | fem.                                                                 | —                                                |
| Network                                       | rede                                           | fem.                                                                 | —                                                |
| Volume                                        | volume                                         | masc.                                                                | —                                                |
| Registry                                      | registro                                       | masc.                                                                | —                                                |
| Snapshot                                      | snapshot                                       | masc. (o snapshot)                                                   | instantâneo                                      |
| Trust token                                   | token de confiança                             | masc.                                                                | —                                                |
| Namespace (containerd/Nomad)                  | namespace                                      | masc.; Kubernetes "Namespace" capitalised                            | espaço de nomes                                  |
| deploy                                        | implantar / implantação                        | verb / fem. noun                                                     | deployar, publicar                               |
| redeploy                                      | reimplantar / reimplantação                    |                                                                      |                                                  |
| pull (image / git)                            | baixar (image pull: "baixar"; noun "download") |                                                                      | puxar                                            |
| push (image / git)                            | enviar                                         |                                                                      | empurrar                                         |
| prune                                         | limpar / limpeza                               |                                                                      | podar                                            |
| drain (node)                                  | drenar                                         |                                                                      | esvaziar                                         |
| Nomad job, allocation, evaluation, task group | job, alocação, avaliação, grupo de tarefas     | job kept in English (masc.); others translated                       | Stack (for a job), contêiner (for an allocation) |

## Further decisions made while translating

| English term                                | pt-BR term                                         | Grammatical notes                                       | Words to avoid              |
| ------------------------------------------- | -------------------------------------------------- | ------------------------------------------------------- | --------------------------- |
| Delete (Kubernetes objects, volumes, Incus) | Excluir                                            | as Windows pt-BR; "Remove" stays "Remover"              | Deletar, Apagar             |
| Workload(s)                                 | carga(s) de trabalho                               | fem.; as the Kubernetes pt-BR docs                      | workload                    |
| Kubernetes Node                             | Node                                               | kept, capitalised; Swarm/Nomad node = nó                | —                           |
| Swarm secret / config                       | secret / config                                    | masc. (o secret, o config), kept as Docker object names | segredo (for these objects) |
| Swarm manager / worker                      | manager / worker                                   | masc., kept as Docker terms                             | gerente, trabalhador        |
| Build (verb/noun)                           | fazer build / a build                              | fem.                                                    | compilar                    |
| Commit (container to image)                 | fazer commit                                       | masc. noun (o commit)                                   | confirmar                   |
| Label                                       | rótulo                                             | masc.                                                   | etiqueta                    |
| Healthcheck                                 | verificação de integridade                         | fem.; "unhealthy" = não íntegro                         | saúde                       |
| Passphrase                                  | senha                                              | fem.; "Passphrase" of an SSH key = "Senha da chave"     | frase secreta               |
| Fingerprint                                 | impressão digital                                  | fem.                                                    | —                           |
| Keychain (OS)                               | chaveiro do sistema                                | masc.                                                   | —                           |
| Toasts / results                            | past participle after the name ("{name} removido") | agrees with the object's gender                         | —                           |
