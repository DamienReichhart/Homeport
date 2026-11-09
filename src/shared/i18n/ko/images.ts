/** Korean text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': '이미지',
  'images.title': '이미지',
  'images.summary': '{count, plural, other {이미지 #개}} · {size}',
  'images.cleanUp': '정리',
  'images.cleanUp.dangling': '댕글링 이미지',
  'images.cleanUp.all': '사용하지 않는 모든 이미지',
  'images.saveSelected': '{count, plural, =1 {이미지 저장…} other {이미지 #개 저장…}}',
  'images.load': '불러오기…',
  'images.importTar': 'tar에서 가져오기…',
  'images.build': '빌드',
  'images.pull': '이미지 풀',
  'images.search': '이미지 검색',
  'images.empty.title': '이미지 없음',
  'images.empty.description': '시작하려면 레지스트리에서 이미지를 풀하세요.',

  'images.column.name': '이름',
  'images.column.id': 'ID',
  'images.column.created': '생성일',
  'images.column.size': '크기',
  'images.column.status': '상태',
  'images.moreTags': '태그 {count}개 더 있음',
  'images.inUse': '사용 중 · {count}',
  'images.unused': '사용 안 함',
  'images.runOne': '{name} 실행',
  'images.action.inspect': '검사',
  'images.action.tag': '태그',
  'images.action.push': '푸시',
  'images.action.save': '저장…',

  'images.remove.title': '{name} 이미지를 삭제하시겠습니까?',
  'images.remove.everyName':
    '이미지의 모든 이름({names})과 다른 곳에서 사용하지 않는 레이어가 삭제됩니다.',
  'images.remove.description':
    '이미지와 다른 이미지가 사용하지 않는 레이어가 삭제됩니다. 나중에 다시 풀할 수 있습니다.',
  'images.prune.allTitle': '사용하지 않는 모든 이미지를 삭제하시겠습니까?',
  'images.prune.danglingTitle': '댕글링 이미지를 삭제하시겠습니까?',
  'images.prune.allDescription':
    '태그가 있는 이미지를 포함해, 컨테이너가 사용하지 않는 모든 이미지가 삭제됩니다.',
  'images.prune.danglingDescription': '빌드와 풀 후에 남은 태그 없는 이미지가 삭제됩니다.',

  'images.removed': '{name} 이미지를 삭제했습니다',
  'images.pruned': '{count, plural, other {이미지 #개}}를 삭제하고 공간 {space}를 확보했습니다',
  'images.pulled': '{reference} 풀 완료',
  'images.pushed': '{reference} 푸시 완료',
  'images.built': '{name} 빌드 완료',
  'images.builtUnnamed': '이미지를 빌드했습니다',
  'images.saved': '{path}에 저장했습니다',
  'images.loadedOne': '{name} 불러옴',
  'images.loaded': '{count, plural, other {이미지 #개}}를 불러왔습니다',
  'images.imported': '{name} 가져옴',

  'images.files.save': '이미지를 저장할 위치',
  'images.files.load': '이미지를 불러올 파일',
  'images.files.import': '파일 시스템을 가져올 파일',
  'images.files.imageArchives': '이미지 아카이브',
  'images.files.imageArchive': '이미지 아카이브',
  'images.files.filesystemArchives': '파일 시스템 아카이브',

  'images.buildDialog.title': '이미지 빌드',
  'images.buildDialog.description':
    '이 컴퓨터의 폴더에서 빌드합니다. Homeport가 .dockerignore에서 제외한 항목을 빼고 폴더를 호스트로 보내면 호스트의 엔진이 빌드합니다.',
  'images.buildDialog.built': '✓ 빌드됨',
  'images.buildDialog.folder': '폴더',
  'images.buildDialog.chooseFolder': '빌드할 폴더 선택',
  'images.buildDialog.choose': '선택…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': '대상 스테이지',
  'images.buildDialog.targetPlaceholder': '마지막 스테이지',
  'images.buildDialog.names': '이름',
  'images.buildDialog.buildArgs': '빌드 인수',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0, 한 줄에 하나씩',
  'images.buildDialog.noCache': '빌드 캐시 사용 안 함',
  'images.buildDialog.pull': '새 기본 이미지 풀',
  'images.buildForm.noFolder': '빌드할 폴더를 선택하세요.',
  'images.buildForm.argNeedsName': '“{line}”에는 “=” 앞에 이름이 필요합니다.',

  'images.hub.unavailable':
    '지금은 Docker Hub를 검색할 수 없습니다: {error} nginx:1.27 같은 전체 이름으로는 계속 풀할 수 있습니다.',
  'images.hub.noMatch': 'Docker Hub에 “{term}” 검색어와 일치하는 항목이 없습니다.',
  'images.hub.results': 'Docker Hub 결과',
  'images.hub.result': '{name}, 별 {stars}개',
  'images.hub.resultOfficial': '{name}, 공식 이미지, 별 {stars}개',
  'images.hub.official': '공식',
  'images.hub.backToSearch': '검색',
  'images.hub.tagCount': '{count, plural, other {태그 #개}}',
  'images.hub.filterTags': '태그 필터',
  'images.hub.tags': '태그',
  'images.hub.noTagMatch': '“{filter}”에 일치하는 태그가 없습니다.',
  'images.hub.moreTags': '태그 더 보기',

  'images.importDialog.title': '파일 시스템을 이미지로 가져오기',
  'images.importDialog.description':
    '{path} 파일은 자체 기록이나 설정이 없는 단일 레이어 이미지가 됩니다. 실행할 수 있게 하려면 아래에 명령을 지정하세요.',
  'images.importDialog.name': '이미지 이름',
  'images.importDialog.changes': '변경 사항',
  'images.importDialog.changesHint':
    '한 줄에 하나씩 Dockerfile 명령어: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME 또는 WORKDIR.',
  'images.importDialog.import': '가져오기',

  'images.inspect.layers': '레이어',
  'images.inspect.raw': '원본 JSON',
  'images.inspect.created': '생성일',
  'images.inspect.instruction': '명령어',
  'images.inspect.size': '크기',

  'images.pullDialog.title': '이미지 풀',
  'images.pullDialog.description':
    'Docker Hub에서 이름으로 검색하거나, 호스트가 접근할 수 있는 참조를 입력하세요(예: nginx:1.27 또는 ghcr.io/org/app:v2).',
  'images.pullDialog.placeholder': 'postgres 또는 nginx:1.27',
  'images.pullDialog.reference': '이미지 참조',
  'images.pullDialog.pull': '풀',
  'images.done': '완료되었습니다.',

  'images.pushDialog.title': '{name} 푸시',
  'images.pushDialog.description':
    '이름이 가리키는 레지스트리로 푸시합니다(예: ghcr.io/org/app:v2). org/app:v2 같은 이름은 Docker Hub로 푸시합니다.',
  'images.pushDialog.target': '푸시할 이름',
  'images.pushDialog.push': '푸시',
  'images.pushDialog.tagsFirst': '태그와 마찬가지로, 먼저 이미지에 이 이름을 붙입니다.',
  'images.pushDialog.uploading': '업로드하는 중 {current} / {total}',
  'images.push.pushed': '{pushed, plural, other {레이어 #개}} 푸시됨',
  'images.push.pushedAndExisting':
    '{pushed, plural, other {레이어 #개}} 푸시됨, {existing}개는 이미 레지스트리에 있음',

  'images.tagDialog.title': '{name} 태그',
  'images.tagDialog.newName': '새 이름',
  'images.tagDialog.add': '태그 추가',

  'images.transfer.savingOne': '{name} 저장하는 중',
  'images.transfer.saving': '{count, plural, other {이미지 #개}} 저장하는 중',
  'images.transfer.to': '대상: {path}',
  'images.transfer.from': '원본: {path}',
  'images.transfer.written': '기록됨',
  'images.transfer.sent': '전송됨',
  'images.transfer.loading': '이미지 불러오는 중',
  'images.transfer.importing': '{reference} 가져오는 중',
  'images.transfer.doneWith': '완료: {images}',
  'images.palette.pull': '이미지 풀',
} satisfies Translation<typeof en>;
