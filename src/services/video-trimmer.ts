/**
 * Video Trimmer Service
 * Trims videos larger than 10MB to fit within the limit by keeping only the first portion
 */

const MAX_VIDEO_SIZE = 10 * 1024 * 1024 // 10MB in bytes

/**
 * Trims a video file to fit within 10MB by keeping only the first portion
 * @param file - The video file to trim
 * @returns Promise<File> - The trimmed video file
 */
export async function trimVideoTo10MB(file: File): Promise<File> {
  // If file is already under 10MB, return as is
  if (file.size <= MAX_VIDEO_SIZE) {
    return file
  }

  return new Promise((resolve, reject) => {
    const video = document.createElement('video')
    const url = URL.createObjectURL(file)

    video.src = url
    video.preload = 'metadata'
    video.muted = true // Mute to avoid audio issues during processing
    video.playsInline = true

    video.onloadedmetadata = () => {
      const duration = video.duration // in seconds
      const originalSize = file.size // in bytes
      const sizePerSecond = originalSize / duration // bytes per second

      // Calculate how many seconds we can keep to fit in 10MB
      // Use 90% to leave some margin
      const maxDuration = (MAX_VIDEO_SIZE / sizePerSecond) * 0.9
      const targetDuration = Math.min(duration, maxDuration)

      // Create a canvas to capture video frames
      const canvas = document.createElement('canvas')
      canvas.width = video.videoWidth
      canvas.height = video.videoHeight
      const ctx = canvas.getContext('2d')

      if (!ctx) {
        URL.revokeObjectURL(url)
        reject(new Error('Could not get canvas context'))
        return
      }

      // Create MediaRecorder to record the trimmed video
      const chunks: Blob[] = []
      let mediaRecorder: MediaRecorder | null = null
      let animationFrameId: number | null = null
      let stopTimeoutId: number | null = null

      const cleanup = () => {
        URL.revokeObjectURL(url)
        video.pause()
        video.src = ''
        if (animationFrameId !== null) {
          cancelAnimationFrame(animationFrameId)
        }
        if (stopTimeoutId !== null) {
          clearTimeout(stopTimeoutId)
        }
      }

      const startRecording = () => {
        try {
          // Create a MediaStream from the canvas
          const stream = canvas.captureStream(30) // 30 fps

          // Determine the best supported mime type
          let mimeType = 'video/webm;codecs=vp9,opus'
          if (!MediaRecorder.isTypeSupported(mimeType)) {
            mimeType = 'video/webm;codecs=vp8,opus'
          }
          if (!MediaRecorder.isTypeSupported(mimeType)) {
            mimeType = 'video/webm'
          }
          if (!MediaRecorder.isTypeSupported(mimeType)) {
            mimeType = 'video/mp4'
          }

          // Calculate bitrate to target ~9MB for the target duration
          const targetBitrate = Math.floor((MAX_VIDEO_SIZE * 8 * 0.9) / targetDuration) // bits per second

          mediaRecorder = new MediaRecorder(stream, {
            mimeType,
            videoBitsPerSecond: Math.min(targetBitrate, 2500000), // Cap at 2.5 Mbps
          })

          mediaRecorder.ondataavailable = (event) => {
            if (event.data && event.data.size > 0) {
              chunks.push(event.data)
            }
          }

          mediaRecorder.onstop = () => {
            cleanup()
            const blob = new Blob(chunks, { type: mediaRecorder?.mimeType || 'video/webm' })
            const trimmedFile = new File([blob], file.name, {
              type: blob.type,
              lastModified: Date.now(),
            })

            // If still too large, try with lower bitrate
            if (trimmedFile.size > MAX_VIDEO_SIZE) {
              trimWithLowerBitrate(file, targetDuration * 0.8)
                .then(resolve)
                .catch(reject)
            } else {
              resolve(trimmedFile)
            }
          }

          mediaRecorder.onerror = () => {
            cleanup()
            reject(new Error('MediaRecorder error'))
          }

          // Start recording
          mediaRecorder.start(1000) // Collect data every second

          // Set up video playback and frame drawing
          video.currentTime = 0

          const drawFrame = () => {
            if (video.ended || video.paused || video.currentTime >= targetDuration) {
              if (mediaRecorder && mediaRecorder.state !== 'inactive') {
                mediaRecorder.stop()
              }
              return
            }

            try {
              ctx.drawImage(video, 0, 0, canvas.width, canvas.height)
              animationFrameId = requestAnimationFrame(drawFrame)
            } catch (error) {
              console.error('Error drawing frame:', error)
              if (mediaRecorder && mediaRecorder.state !== 'inactive') {
                mediaRecorder.stop()
              }
            }
          }

          video.oncanplay = () => {
            video.play().catch((error) => {
              console.error('Error playing video:', error)
              cleanup()
              reject(new Error('Error playing video'))
            })
            drawFrame()
          }

          // Stop after target duration (with some buffer)
          stopTimeoutId = window.setTimeout(
            () => {
              if (mediaRecorder && mediaRecorder.state === 'recording') {
                mediaRecorder.stop()
              }
              video.pause()
            },
            targetDuration * 1000 + 500,
          ) // Add 500ms buffer
        } catch (error) {
          cleanup()
          reject(error)
        }
      }

      video.oncanplay = startRecording

      video.onerror = () => {
        cleanup()
        reject(new Error('Error loading video'))
      }
    }

    video.onerror = () => {
      URL.revokeObjectURL(url)
      reject(new Error('Error loading video metadata'))
    }
  })
}

/**
 * Trim video with lower bitrate if initial attempt was too large
 */
async function trimWithLowerBitrate(file: File, targetDuration: number): Promise<File> {
  return new Promise((resolve, reject) => {
    const video = document.createElement('video')
    const url = URL.createObjectURL(file)

    video.src = url
    video.preload = 'metadata'
    video.muted = true
    video.playsInline = true
    video.crossOrigin = 'anonymous'

    video.onloadedmetadata = () => {
      const canvas = document.createElement('canvas')
      canvas.width = video.videoWidth
      canvas.height = video.videoHeight
      const ctx = canvas.getContext('2d')

      if (!ctx) {
        URL.revokeObjectURL(url)
        reject(new Error('Could not get canvas context'))
        return
      }

      const chunks: Blob[] = []
      const stream = canvas.captureStream(24) // Lower FPS

      // Use lower bitrate
      let mimeType = 'video/webm;codecs=vp8'
      if (!MediaRecorder.isTypeSupported(mimeType)) {
        mimeType = 'video/webm'
      }

      const mediaRecorder = new MediaRecorder(stream, {
        mimeType,
        videoBitsPerSecond: 1000000, // 1 Mbps - lower quality
      })

      let animationFrameId: number | null = null
      let stopTimeoutId: number | null = null

      const cleanup = () => {
        URL.revokeObjectURL(url)
        video.pause()
        video.src = ''
        if (animationFrameId !== null) {
          cancelAnimationFrame(animationFrameId)
        }
        if (stopTimeoutId !== null) {
          clearTimeout(stopTimeoutId)
        }
      }

      mediaRecorder.ondataavailable = (event) => {
        if (event.data && event.data.size > 0) {
          chunks.push(event.data)
        }
      }

      mediaRecorder.onstop = () => {
        cleanup()
        const blob = new Blob(chunks, { type: mediaRecorder.mimeType || 'video/webm' })
        const trimmedFile = new File([blob], file.name, {
          type: blob.type,
          lastModified: Date.now(),
        })

        // If still too large, reduce duration further
        if (trimmedFile.size > MAX_VIDEO_SIZE && targetDuration > 5) {
          // Minimum 5 seconds
          const newDuration = targetDuration * 0.7 // Reduce by 30%
          trimWithLowerBitrate(file, newDuration).then(resolve).catch(reject)
        } else {
          resolve(trimmedFile)
        }
      }

      mediaRecorder.onerror = () => {
        cleanup()
        reject(new Error('MediaRecorder error'))
      }

      mediaRecorder.start(1000)

      video.currentTime = 0

      const drawFrame = () => {
        if (video.ended || video.paused || video.currentTime >= targetDuration) {
          if (mediaRecorder.state !== 'inactive') {
            mediaRecorder.stop()
          }
          return
        }

        try {
          ctx.drawImage(video, 0, 0, canvas.width, canvas.height)
          animationFrameId = requestAnimationFrame(drawFrame)
        } catch (error) {
          console.error('Error drawing frame:', error)
          if (mediaRecorder.state !== 'inactive') {
            mediaRecorder.stop()
          }
        }
      }

      video.oncanplay = () => {
        video.play().catch((error) => {
          console.error('Error playing video:', error)
          cleanup()
          reject(new Error('Error playing video'))
        })
        drawFrame()
      }

      stopTimeoutId = window.setTimeout(
        () => {
          if (mediaRecorder.state === 'recording') {
            mediaRecorder.stop()
          }
          video.pause()
        },
        targetDuration * 1000 + 500,
      )

      video.onerror = () => {
        cleanup()
        reject(new Error('Error loading video'))
      }
    }

    video.onerror = () => {
      URL.revokeObjectURL(url)
      reject(new Error('Error loading video metadata'))
    }
  })
}
