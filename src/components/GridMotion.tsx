import { useEffect, useRef, useState } from 'react';
import type { FC, ReactNode } from 'react';
import { gsap } from 'gsap';

function cn(...classes: (string | undefined | null | boolean)[]): string {
  return classes.filter(Boolean).join(' ');
}

interface GridMotionProps {
  items?: (string | ReactNode)[];
  gradientColor?: string;
  onItemClick?: (index: number, item: string | ReactNode) => void;
  scrollEnabled?: boolean;
}

const GridMotion: FC<GridMotionProps> = ({ items = [], gradientColor = 'black', onItemClick, scrollEnabled = false }) => {
  const gridRef = useRef<HTMLDivElement>(null);
  const rowRefs = useRef<(HTMLDivElement | null)[]>([]);
  const mouseXRef = useRef<number>(typeof window !== 'undefined' ? window.innerWidth / 2 : 0);
  const scrollIndexRef = useRef<number>(0);
  const isScrollingRef = useRef<boolean>(false);
  const [currentScrollIndex, setCurrentScrollIndex] = useState<number>(0);

  const totalItems = 28;
  const defaultItems = Array.from({ length: totalItems }, (_, index) => `Item ${index + 1}`);
  
  // Use scroll index to determine which items to show
  const getDisplayItems = () => {
    if (scrollEnabled && items.length > totalItems) {
      const startIndex = currentScrollIndex;
      const endIndex = Math.min(startIndex + totalItems, items.length);
      const displayItems = items.slice(startIndex, endIndex);
      
      // If we don't have enough items, fill with defaults
      while (displayItems.length < totalItems) {
        displayItems.push(...defaultItems.slice(0, totalItems - displayItems.length));
      }
      
      return displayItems.slice(0, totalItems);
    }
    
    return items.length > 0 ? items.slice(0, totalItems) : defaultItems;
  };
  
  const combinedItems = getDisplayItems();

  useEffect(() => {
    if (typeof window === 'undefined') return;
    
    gsap.ticker.lagSmoothing(0);

    const handleMouseMove = (e: MouseEvent): void => {
      mouseXRef.current = e.clientX;
    };

    const updateMotion = (): void => {
      const maxMoveAmount = 300;
      const baseDuration = 0.8;
      const inertiaFactors = [0.6, 0.4, 0.3, 0.2];

      rowRefs.current.forEach((row, index) => {
        if (row) {
          const direction = index % 2 === 0 ? 1 : -1;
          const moveAmount = ((mouseXRef.current / window.innerWidth) * maxMoveAmount - maxMoveAmount / 2) * direction;

          gsap.to(row, {
            x: moveAmount,
            duration: baseDuration + inertiaFactors[index % inertiaFactors.length],
            ease: 'power3.out',
            overwrite: 'auto'
          });
        }
      });
    };

    const removeAnimationLoop = gsap.ticker.add(updateMotion);
    window.addEventListener('mousemove', handleMouseMove);

    // Scroll functionality
    if (scrollEnabled && items.length > totalItems) {
      let scrollTimeout: number;
      
      const handleWheel = (e: WheelEvent): void => {
        e.preventDefault();
        
        if (isScrollingRef.current) return;
        
        const scrollDirection = e.deltaY > 0 ? 1 : -1;
        const maxScrollIndex = Math.max(0, items.length - totalItems);
        
        scrollIndexRef.current = Math.max(0, Math.min(maxScrollIndex, scrollIndexRef.current + scrollDirection));
        
        // Update the state to trigger re-render
        setCurrentScrollIndex(scrollIndexRef.current);
        
        // Update grid items with animation
        const startIndex = scrollIndexRef.current;
        const newItems = items.slice(startIndex, startIndex + totalItems);
        
        if (newItems.length === totalItems) {
          isScrollingRef.current = true;
          
          // Animate the transition
          gsap.to(rowRefs.current, {
            opacity: 0,
            y: scrollDirection > 0 ? -50 : 50,
            duration: 0.3,
            onComplete: () => {
              // Force re-render with new items
              const gridContainer = gridRef.current;
              if (gridContainer) {
                gridContainer.style.opacity = '0';
                setTimeout(() => {
                  gridContainer.style.opacity = '1';
                }, 50);
              }
              
              gsap.to(rowRefs.current, {
                opacity: 1,
                y: 0,
                duration: 0.3,
                onComplete: () => {
                  isScrollingRef.current = false;
                }
              });
            }
          });
        }
        
        clearTimeout(scrollTimeout);
        scrollTimeout = window.setTimeout(() => {
          isScrollingRef.current = false;
        }, 1000);
      };

      window.addEventListener('wheel', handleWheel, { passive: false });

      return () => {
        window.removeEventListener('mousemove', handleMouseMove);
        window.removeEventListener('wheel', handleWheel);
        removeAnimationLoop();
        clearTimeout(scrollTimeout);
      };
    }

    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      removeAnimationLoop();
    };
  }, [items, scrollEnabled]);

  return (
    <div ref={gridRef} className={cn('w-full', 'h-full', 'overflow-hidden')}>
      <section
        className={cn('relative', 'flex', 'justify-center', 'items-center', 'w-full', 'h-screen', 'overflow-hidden')}
        style={{
          background: `radial-gradient(circle, ${gradientColor} 0%, transparent 100%)`
        }}
      >
        <div className={cn('z-[4]', 'absolute', 'inset-0', 'bg-[length:250px]', 'pointer-events-none')}></div>
        <div className={cn('z-[2]', 'relative', 'flex-none', 'gap-4', 'grid', 'grid-cols-1', 'grid-rows-4', 'w-[150vw]', 'h-[150vh]', 'rotate-[-15deg]', 'origin-center')}>
          {Array.from({ length: 4 }, (_, rowIndex) => (
            <div
              key={rowIndex}
              className={cn('gap-4', 'grid', 'grid-cols-7')}
              style={{ willChange: 'transform, filter' }}
              ref={el => {
                if (el) rowRefs.current[rowIndex] = el;
              }}
            >
              {Array.from({ length: 7 }, (_, itemIndex) => {
                const content = combinedItems[rowIndex * 7 + itemIndex];
                const globalIndex = rowIndex * 7 + itemIndex;
                const isImage = typeof content === 'string' && (content.startsWith('http') || content.startsWith('/') || content.includes('.'));

                return (
                  <div key={itemIndex} className="relative">
                    <div 
                      className={cn('relative', 'flex', 'justify-center', 'items-center', 'bg-[#111]', 'rounded-[10px]', 'w-full', 'h-full', 'overflow-hidden', 'text-[1.5rem]', 'text-white', 'cursor-pointer', 'hover:scale-105', 'transition-transform', 'duration-300')}
                      onClick={() => onItemClick && onItemClick(globalIndex, content)}
                    >
                      {isImage ? (
                        <img
                          src={content}
                          alt={`Grid item ${globalIndex}`}
                          className={cn('w-full', 'h-full', 'object-cover')}
                        />
                      ) : (
                        <div className={cn('z-1', 'p-4', 'text-center')}>{content}</div>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          ))}
        </div>
        <div className={cn('top-0', 'left-0', 'relative', 'w-full', 'h-full', 'pointer-events-none')}></div>
      </section>
    </div>
  );
};

export default GridMotion;
