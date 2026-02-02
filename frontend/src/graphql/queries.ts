import { gql } from 'graphql-request'

export const EVALUATIONS_QUERY = gql`
  query GetEvaluations($search: String, $categoryId: String, $status: String, $limit: Int, $offset: Int) {
    evaluations(search: $search, categoryId: $categoryId, status: $status, limit: $limit, offset: $offset) {
      id
      title
      status
      category_id
      created_at
      scale_type
    }
  }
`

export const EVALUATION_DETAIL_QUERY = gql`
  query GetEvaluation($id: String!) {
    evaluation(id: $id) {
      id
      title
      description
      status
      scale_type
      questions {
        id
        text
        metadata
      }
    }
  }
`

export const CATEGORIES_QUERY = gql`
  query GetCategories($userId: String!) {
    categories(userId: $userId) {
      id
      name
      color
    }
  }
`
