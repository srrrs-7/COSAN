package queue

import (
	"context"
	"fmt"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/sqs"
	"github.com/aws/aws-sdk-go-v2/service/sqs/types"
)

// SQSHandler は、SQSキューを操作するための構造体です。
type SQSHandler struct {
	client   *sqs.Client
	queueURL string
}

// NewSQSHandler は、SQSHandlerの新しいインスタンスを作成します。
func NewSQSHandler(queueName string) (*SQSHandler, error) {
	// Load the Shared AWS Configuration (~/.aws/config)
	cfg, err := config.LoadDefaultConfig(context.TODO())
	if err != nil {
		return nil, fmt.Errorf("loading AWS config: %w", err)
	}

	// Create an Amazon SQS client
	client := sqs.NewFromConfig(cfg)

	// Get the queue URL
	result, err := client.GetQueueUrl(context.TODO(), &sqs.GetQueueUrlInput{
		QueueName: aws.String(queueName),
	})
	if err != nil {
		return nil, fmt.Errorf("getting queue URL: %w", err)
	}

	return &SQSHandler{
		client:   client,
		queueURL: *result.QueueUrl,
	}, nil
}

// Enqueue は、メッセージをキューにエンキューします。
func (h *SQSHandler) Enqueue(ctx context.Context, messageBody string) error {
	_, err := h.client.SendMessage(ctx, &sqs.SendMessageInput{
		MessageBody: aws.String(messageBody),
		QueueUrl:    aws.String(h.queueURL),
	})
	if err != nil {
		return fmt.Errorf("sending message: %w", err)
	}

	return nil
}

// Dequeue は、キューからメッセージをデキューします。
func (h *SQSHandler) Dequeue(ctx context.Context) (*types.Message, error) {
	result, err := h.client.ReceiveMessage(ctx, &sqs.ReceiveMessageInput{
		QueueUrl:            aws.String(h.queueURL),
		MaxNumberOfMessages: int32(1),
		WaitTimeSeconds:     int32(20), // Long polling for 20 seconds
	})
	if err != nil {
		return nil, fmt.Errorf("receiving message: %w", err)
	}

	if len(result.Messages) == 0 {
		return nil, nil // No messages available
	}

	return &result.Messages[0], nil
}

// DeleteMessage は、キューからメッセージを削除します。
func (h *SQSHandler) DeleteMessage(ctx context.Context, receiptHandle string) error {
	_, err := h.client.DeleteMessage(ctx, &sqs.DeleteMessageInput{
		QueueUrl:      aws.String(h.queueURL),
		ReceiptHandle: aws.String(receiptHandle),
	})
	if err != nil {
		return fmt.Errorf("deleting message: %w", err)
	}

	return nil
}

// DelQueue は、キューを削除します。
func (h *SQSHandler) DelQueue(ctx context.Context) error {
	_, err := h.client.DeleteQueue(ctx, &sqs.DeleteQueueInput{
		QueueUrl: aws.String(h.queueURL),
	})
	if err != nil {
		return fmt.Errorf("deleting queue: %w", err)
	}

	return nil
}

func (h *SQSHandler) ExtendVisibility(ctx context.Context, receiptHandle string, visibilityTimeout int32) error {
	_, err := h.client.ChangeMessageVisibility(ctx, &sqs.ChangeMessageVisibilityInput{
		QueueUrl:          aws.String(h.queueURL),
		ReceiptHandle:     aws.String(receiptHandle),
		VisibilityTimeout: int32(visibilityTimeout),
	})
	if err != nil {
		return fmt.Errorf("extending message visibility: %w", err)
	}

	return nil
}
