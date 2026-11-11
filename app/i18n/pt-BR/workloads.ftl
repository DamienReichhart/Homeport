### The `workloads` namespace, in Português (Brasil) (pt-BR).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/pt-BR/workloads.ts:
### change the text there, then run it again.

workloads-nav = Cargas de trabalho
workloads-title = Cargas de trabalho
workloads-summary = { $count ->
        [one] { NUM($count) } carga de trabalho
       *[other] { NUM($count) } cargas de trabalho
    }
workloads-search = Pesquisar cargas de trabalho
workloads-empty = Nenhuma aqui.
workloads-kind-deployment = Deployments
workloads-kind-statefulSet = StatefulSets
workloads-kind-daemonSet = DaemonSets
workloads-kind-job = Jobs
workloads-kind-cronJob = CronJobs
workloads-one-deployment = Deployment
workloads-one-statefulSet = StatefulSet
workloads-one-daemonSet = DaemonSet
workloads-one-job = Job
workloads-one-cronJob = CronJob
workloads-column-name = Nome
workloads-column-namespace = Namespace
workloads-column-health = Integridade
workloads-column-ready = Prontos
workloads-column-completions = Conclusões
workloads-column-active = Ativos
workloads-column-upToDate = Atualizados
workloads-column-schedule = Agendamento
workloads-column-lastSchedule = Última execução
workloads-column-images = Imagens
workloads-column-age = Idade
workloads-tab-overview = Visão geral
workloads-tab-pods = Pods
workloads-tab-jobs = Jobs
workloads-tab-logs = Logs
workloads-tab-conditions = Condições
workloads-tab-events = Eventos
workloads-tab-source = YAML
workloads-detail-namespace = Namespace
workloads-detail-ready = Prontos
workloads-detail-upToDate = Atualizados
workloads-detail-schedule = Agendamento
workloads-detail-lastSchedule = Última execução
workloads-detail-strategy = Estratégia
workloads-detail-selector = Seleciona Pods por
workloads-detail-owner = Controlado por
workloads-detail-age = Idade
workloads-detail-labels = Rótulos
workloads-detail-annotations = Anotações
workloads-detail-containers = Contêineres
workloads-detail-noJobs = Ele não iniciou nenhum Job que ainda esteja mantido.
workloads-container-name = Contêiner
workloads-container-image = Imagem
workloads-container-ports = Portas
workloads-action-menu = Ações de { $name }
workloads-action-scale = Escalar
workloads-action-restart = Reiniciar
workloads-action-trigger = Executar agora
workloads-action-suspend = Suspender
workloads-action-resume = Retomar
workloads-scale-title = Escalar { $name }
workloads-scale-description = No namespace { $namespace } em { $cluster }. Ele executa { $current ->
        [one] { NUM($current) } Pod
       *[other] { NUM($current) } Pods
    } agora.
workloads-scale-replicas = Pods
workloads-scale-autoscaled = O autoscaler { $autoscaler } gerencia os Pods dele, então vai restaurar o número na próxima verificação.
workloads-scale-toZero = Com 0 Pods ele deixa de atender até que você o escale novamente.
workloads-scale-submit = Escalar
workloads-scale-done = { $name } escalado para { $replicas ->
        [one] { NUM($replicas) } Pod
       *[other] { NUM($replicas) } Pods
    }
workloads-scale-failed = O Homeport não conseguiu escalar { $name }
workloads-restart-title = Reiniciar { $name }?
workloads-restart-description = Os Pods dele no namespace { $namespace } em { $cluster } são substituídos um após o outro, conforme a estratégia de rollout, como o kubectl rollout restart.
workloads-restart-confirm = Reiniciar
workloads-restart-done = Reiniciando { $name }
workloads-restart-failed = O Homeport não conseguiu reiniciar { $name }
workloads-suspend-title = Suspender { $name }?
workloads-suspend-description = Ele não inicia mais Jobs no namespace { $namespace } em { $cluster } até que você o retome. Os Jobs já em execução continuam.
workloads-suspend-confirm = Suspender
workloads-suspend-done = { $name } suspenso
workloads-suspend-failed = O Homeport não conseguiu suspender { $name }
workloads-resume-done = { $name } retomado
workloads-resume-failed = O Homeport não conseguiu retomar { $name }
workloads-trigger-done = Job { $job } iniciado
workloads-trigger-failed = O Homeport não conseguiu iniciar um Job a partir de { $name }
