import { compareMessages } from './locales';

const english = {
  'images.remove.title': 'Remove {name}?',
  'images.summary': '{count, plural, one {# image} other {# images}} · {size}',
  'volumes.remove.warning': 'This deletes <strong>{name}</strong>.',
};

describe('the cross-language catalogue check', () => {
  it('passes a complete translation', () => {
    const french = {
      'images.remove.title': 'Supprimer {name} ?',
      'images.summary': '{count, plural, one {# image} other {# images}} · {size}',
      'volumes.remove.warning': 'Cela supprime <strong>{name}</strong>.',
    };
    expect(compareMessages('fr', 'images.ts', english, french)).toEqual([]);
  });

  it('names missing and extra keys, lost arguments and tags, and bad ICU', () => {
    const problems = compareMessages('fr', 'images.ts', english, {
      'images.remove.title': 'Supprimer {nom} ?',
      'images.summary': '{count, plural, one {# image} other {# images}',
      'images.stray': 'Rien',
    });
    expect(problems).toEqual([
      'fr/images.ts: images.remove.title names {nom}, English names {name}',
      expect.stringContaining("fr/images.ts: images.summary isn't valid ICU"),
      'fr/images.ts lacks volumes.remove.warning (English: "This deletes <strong>{name}</strong>.")',
      "fr/images.ts has images.stray, which English doesn't",
    ]);
    expect(
      compareMessages('fr', 'volumes.ts', english, {
        ...english,
        'volumes.remove.warning': 'Cela supprime {name}.',
      }),
    ).toEqual(['fr/volumes.ts: volumes.remove.warning has tags <none>, English has <strong>']);
  });

  it('asks for every plural category the language needs', () => {
    const russian = {
      ...english,
      'images.summary': '{count, plural, one {# образ} other {# образов}} · {size}',
    };
    expect(compareMessages('ru', 'images.ts', english, russian)).toEqual([
      "ru/images.ts: images.summary's plural {count} lacks many, few for ru",
    ]);
    const japanese = {
      ...english,
      'images.summary': '{count, plural, other {# 個のイメージ}} · {size}',
    };
    expect(compareMessages('ja', 'images.ts', english, japanese)).toEqual([]);
  });
});
