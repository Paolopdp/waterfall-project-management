import { useState } from 'react';
import { useTranslation } from 'next-i18next';
import { Tag } from '@/types/forum';

interface TagFilterProps {
  tags: Tag[];
  onTagsChange: (selectedTags: string[]) => void;
}

export default function TagFilter({ tags, onTagsChange }: TagFilterProps) {
  const { t } = useTranslation('common');
  const [selectedTags, setSelectedTags] = useState<string[]>([]);

  const handleTagClick = (tagId: string) => {
    const newSelectedTags = selectedTags.includes(tagId)
      ? selectedTags.filter(id => id !== tagId)
      : [...selectedTags, tagId];
    
    setSelectedTags(newSelectedTags);
    onTagsChange(newSelectedTags);
  };

  return (
    <div>
      <h3 className="text-sm font-medium text-gray-700 mb-2">
        {t('forum.filterByTags')}
      </h3>
      <div className="flex flex-wrap gap-2">
        {tags.map(tag => (
          <button
            key={tag.id}
            onClick={() => handleTagClick(tag.id)}
            className={`px-3 py-1 rounded-full text-sm transition-colors ${
              selectedTags.includes(tag.id)
                ? 'bg-blue-500 text-white'
                : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
            }`}
          >
            {tag.name}
          </button>
        ))}
      </div>
    </div>
  );
}