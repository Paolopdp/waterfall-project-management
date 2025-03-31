export interface Tag {
  id: string;
  name: string;
}

export interface Thread {
  id: string;
  title: string;
  content: string;
  authorId: string;
  author: {
    id: string;
    name: string;
  };
  tags: Tag[];
  createdAt: string;
  updatedAt: string;
}

export interface Comment {
  id: string;
  content: string;
  authorId: string;
  author: {
    id: string;
    name: string;
  };
  threadId: string;
  createdAt: string;
  updatedAt: string;
}
