const router = express.Router();

// Dataset Core Operations
router.get('/');               // Get all datasets with pagination & filters
router.get('/:datasetId');     // Get single dataset by ID
router.post('/');              // Create new dataset
router.put('/:datasetId');     // Update dataset
router.delete('/:datasetId');  // Delete dataset

// Dataset Workflow
router.post('/:datasetId/submit');   // Submit dataset for approval
router.post('/:datasetId/approve');  // Approve dataset
router.post('/:datasetId/reject');   // Reject dataset

// Dataset Comments
router.get('/:datasetId/comments');   // Get all comments for a dataset
router.post('/:datasetId/comments');  // Add a comment to a dataset