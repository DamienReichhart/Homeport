### The `workloads` namespace, in हिन्दी (hi).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/hi/workloads.ts:
### change the text there, then run it again.

workloads-nav = वर्कलोड
workloads-title = वर्कलोड
workloads-summary = { $count ->
        [one] { NUM($count) } वर्कलोड
       *[other] { NUM($count) } वर्कलोड
    }
workloads-search = वर्कलोड खोजें
workloads-empty = यहाँ कुछ नहीं है।
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
workloads-column-name = नाम
workloads-column-namespace = Namespace
workloads-column-health = हेल्थ
workloads-column-ready = तैयार
workloads-column-completions = पूर्ण
workloads-column-active = सक्रिय
workloads-column-upToDate = अप-टू-डेट
workloads-column-schedule = शेड्यूल
workloads-column-lastSchedule = आखिरी रन
workloads-column-images = इमेज
workloads-column-age = उम्र
workloads-tab-overview = अवलोकन
workloads-tab-pods = Pods
workloads-tab-jobs = Jobs
workloads-tab-logs = लॉग
workloads-tab-conditions = कंडीशन
workloads-tab-events = इवेंट
workloads-tab-source = YAML
workloads-detail-namespace = Namespace
workloads-detail-ready = तैयार
workloads-detail-upToDate = अप-टू-डेट
workloads-detail-schedule = शेड्यूल
workloads-detail-lastSchedule = आखिरी रन
workloads-detail-strategy = रणनीति
workloads-detail-selector = Pods को इससे चुनता है
workloads-detail-owner = नियंत्रक
workloads-detail-age = उम्र
workloads-detail-labels = लेबल
workloads-detail-annotations = एनोटेशन
workloads-detail-containers = कंटेनर
workloads-detail-noJobs = इसने ऐसा कोई Job शुरू नहीं किया है जो अभी भी रखा गया हो।
workloads-container-name = कंटेनर
workloads-container-image = इमेज
workloads-container-ports = पोर्ट
workloads-action-menu = { $name } के लिए कार्रवाइयाँ
workloads-action-scale = स्केल करें
workloads-action-restart = फिर से शुरू करें
workloads-action-trigger = अभी चलाएँ
workloads-action-suspend = निलंबित करें
workloads-action-resume = जारी रखें
workloads-scale-title = { $name } स्केल करें
workloads-scale-description = { $cluster } पर Namespace { $namespace } में। अभी यह { $current ->
        [one] { NUM($current) } Pod
       *[other] { NUM($current) } Pod
    } चला रहा है।
workloads-scale-replicas = Pods
workloads-scale-autoscaled = ऑटोस्केलर { $autoscaler } इसके Pods प्रबंधित करता है, इसलिए वह अपनी अगली जाँच पर संख्या वापस सेट कर देगा।
workloads-scale-toZero = 0 Pods के साथ यह तब तक सेवा देना बंद रखता है जब तक आप इसे फिर से स्केल अप न करें।
workloads-scale-submit = स्केल करें
workloads-scale-done = { $name } को { $replicas ->
        [one] { NUM($replicas) } Pod
       *[other] { NUM($replicas) } Pod
    } पर स्केल किया गया
workloads-scale-failed = Homeport { $name } को स्केल नहीं कर सका
workloads-restart-title = { $name } फिर से शुरू करें?
workloads-restart-description = { $cluster } पर Namespace { $namespace } में इसके Pods एक के बाद एक बदले जाते हैं, जैसा इसकी रोलआउट रणनीति कहती है, kubectl rollout restart की तरह।
workloads-restart-confirm = फिर से शुरू करें
workloads-restart-done = { $name } फिर से शुरू हो रहा है
workloads-restart-failed = Homeport { $name } को फिर से शुरू नहीं कर सका
workloads-suspend-title = { $name } निलंबित करें?
workloads-suspend-description = जब तक आप इसे जारी नहीं करते, यह { $cluster } पर Namespace { $namespace } में कोई नया Job शुरू नहीं करेगा। पहले से चल रहे Jobs चलते रहेंगे।
workloads-suspend-confirm = निलंबित करें
workloads-suspend-done = { $name } निलंबित किया गया
workloads-suspend-failed = Homeport { $name } को निलंबित नहीं कर सका
workloads-resume-done = { $name } फिर से जारी किया गया
workloads-resume-failed = Homeport { $name } को जारी नहीं कर सका
workloads-trigger-done = Job { $job } शुरू किया गया
workloads-trigger-failed = Homeport { $name } से Job शुरू नहीं कर सका
