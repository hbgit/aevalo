import { gql } from 'graphql-request'

export const CREATE_EVALUATION_MUTATION = gql`
  mutation CreateEvaluation(
    $userId: String!
    $categoryId: String!
    $title: String!
    $scaleType: String!
  ) {
    createEvaluation(
      userId: $userId
      categoryId: $categoryId
      title: $title
      scaleType: $scaleType
    ) {
      id
      title
      status
    }
  }
`

export const PUBLISH_EVALUATION_MUTATION = gql`
  mutation PublishEvaluation($evaluationId: String!) {
    publishEvaluation(evaluationId: $evaluationId) {
      id
      status
    }
  }
`

export const CLOSE_EVALUATION_MUTATION = gql`
  mutation CloseEvaluation($evaluationId: String!) {
    closeEvaluation(evaluationId: $evaluationId) {
      id
      status
      closed_at
    }
  }
`

export const GENERATE_QUESTIONS_MUTATION = gql`
  mutation GenerateQuestionsWithAI(
    $evaluationId: String!
    $context: String!
  ) {
    generateQuestionsWithAI(
      evaluationId: $evaluationId
      context: $context
    )
  }
`

export const SUBMIT_RESPONSE_MUTATION = gql`
  mutation SubmitResponse(
    $questionId: String!
    $answerValue: String!
  ) {
    submitResponse(
      questionId: $questionId
      answerValue: $answerValue
    ) {
      id
    }
  }
`
