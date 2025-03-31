import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { useTranslation } from 'next-i18next';
import { Tag } from '@/types/forum';

interface ThreadFormProps {
  onSubmit: (data: ThreadFormData) => void;
  onCancel: () => void;
  tags: Tag[];
}

interface ThreadFormData {
  title: string;
  content: string;
  tagIds: string[];
}

export default function ThreadForm({ onSubmit, onCancel, tags }: ThreadFormProps) {
  const { t } = useTranslation('common');
  const { register, handleSubmit, formState: { errors } } = useForm<ThreadFormData>();
  const [selectedTags, setSelectedTags] = useState<string[]>([]);

  const handleTagToggle = (tagId: string) => {
    setSelectedTags(prev => 
      prev.includes(tagId) 
        ? prev.filter(id => id !== tagId)
        : [...prev, tagId]
    );
  };

  const onFormSubmit = (data: ThreadFormData) => {
    onSubmit({
      ...data,
      tagIds: selectedTags
    });
  };

  return (
    <form onSubmit={handleSubmit(onFormSubmit)} className="space-y-4 mb-6">
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          {t('forum.threadTitle')}
        </label>
        <input
          type="text"
          {...register('title', { required: true })}
          className="w-full px-3 py-2 border rounded-md"
        />
        {errors.title && (
          <span className="text-red-500 text-sm">{t('common.required')}</span>
        )}
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          {t('forum.threadContent')}
        </label>
        <textarea
          {...register('content', { required: true })}
          rows={5}
          className="w-full px-3 py-2 border rounded-md"
        />
        {errors.content && (
          <span className="text-red-500 text-sm">{t('common.required')}</span>
        )}
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-1">
          {t('forum.tags')}
        </label>
        <div className="flex flex-wrap gap-2">
          {tags.map(tag => (
            <button
              key={tag.id}
              type="button"
              onClick={() => handleTagToggle(tag.id)}
              className={`px-3 py-1 rounded-full text-sm ${
                selectedTags.includes(tag.id)
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-200 text-gray-700'
              }`}
            >
              {tag.name}
            </button>
          ))}
        </div>
      </div>

      <div className="flex justify-end gap-3">
        <button
          type="button"
          onClick={onCancel}
          className="px-4 py-2 text-gray-600 hover:text-gray-800"
        >
          {t('common.cancel')}
        </button>
        <button
          type="submit"
          className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
        >
          {t('forum.createThread')}
        </button>
      </div>
    </form>
  );
}